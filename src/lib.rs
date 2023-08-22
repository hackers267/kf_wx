/// 客户账号管理
pub mod account;
/// 常量
mod constant;
/// 客服消息
pub mod message;
use std::{io::Error, vec};

use aes::cipher::{
    block_padding::{NoPadding, UnpadError},
    BlockDecryptMut, InvalidLength, KeyIvInit,
};
use base64::{
    alphabet::STANDARD,
    engine::{general_purpose, GeneralPurpose, GeneralPurposeConfig},
    DecodeError, Engine as _,
};
use byteorder::{BigEndian, ReadBytesExt};
use hex::encode;
use serde::Deserialize;
use sha1::{Digest, Sha1};

#[derive(Debug, Deserialize)]
pub struct AccessTokenRes {
    pub errcode: i32,
    pub errmsg: String,
    pub access_token: String,
    pub expires_in: i32,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub token: String,
    pub timestamp: String,
    pub nonce: String,
    pub echostr: String,
}

impl Signature {
    pub fn new(token: &str, timestamp: &str, nonce: &str, echostr: &str) -> Self {
        Signature {
            token: token.to_string(),
            timestamp: timestamp.to_string(),
            nonce: nonce.to_string(),
            echostr: echostr.to_string(),
        }
    }
}

pub fn msg_signature(signature: &Signature) -> String {
    let signature = signature.clone();
    let mut arr = vec![
        signature.token,
        signature.timestamp,
        signature.nonce,
        signature.echostr,
    ];
    arr.sort();
    let str = arr.join("");
    let mut hasher = Sha1::new();
    hasher.update(str);
    let result = hasher.finalize();
    encode(result)
}

const G: GeneralPurpose = GeneralPurpose::new(
    &STANDARD,
    GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true),
);

type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

#[derive(Debug, PartialEq, Eq)]
pub struct Decrypt {
    pub msg: String,
    pub receiveid: String,
}

#[derive(Debug, PartialEq)]
pub enum DecryptErr {
    Std(String),
    UnpadError(String),
    DecodeError(DecodeError),
    InvalidLength(InvalidLength),
}

impl From<UnpadError> for DecryptErr {
    fn from(value: UnpadError) -> Self {
        Self::UnpadError(value.to_string())
    }
}

impl From<DecodeError> for DecryptErr {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<Error> for DecryptErr {
    fn from(value: Error) -> Self {
        Self::Std(value.to_string())
    }
}
impl From<InvalidLength> for DecryptErr {
    fn from(value: InvalidLength) -> Self {
        Self::InvalidLength(value)
    }
}

pub fn decode_msg(msg: &str, encode_ase_key: &str) -> Result<Decrypt, DecryptErr> {
    let result = general_purpose::STANDARD.decode(msg)?;
    let encode_ase_key = format!("{}=", encode_ase_key);
    let ase_key = G.decode(encode_ase_key)?;
    let iv = &ase_key[..16];
    let key = &ase_key[..32];
    let cipher = Aes256CbcDec::new_from_slices(key, iv)?;
    let mut buffer = vec![0u8; 1024];
    let rand_msg = cipher.decrypt_padded_b2b_mut::<NoPadding>(&result, &mut buffer)?;
    let last = rand_msg[rand_msg.len() - 1];
    let rand_msg = &rand_msg[0..rand_msg.len() - last as usize];
    let content = &rand_msg[16..];
    let mut msg_len = &content[0..4];
    let msg_len = msg_len.read_u32::<BigEndian>()? + 4;
    let msg = &content[4..msg_len as usize];
    let receiveid = &content[msg_len as usize..];
    let msg = String::from_utf8_lossy(msg).to_string();
    let receiveid = String::from_utf8_lossy(receiveid).to_string();
    println!("msg:{:?}", msg);
    println!("receiveid:{:?}", receiveid);
    Ok(Decrypt { msg, receiveid })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_msg_signature() {
        let token = "QDG6eK";
        let timestamp = "1409659813";
        let nonce = "1372623149";
        let echostr = "RypEvHKD8QQKFhvQ6QleEB4J58tiPdvo+rtK1I9qca6aM/wvqnLSV5zEPeusUiX5L5X/0lWfrf0QADHHhGd3QczcdCUpj911L3vg3W/sYYvuJTs3TUUkSUXxaccAS0qhxchrRYt66wiSpGLYL42aM6A8dTT+6k4aSknmPj48kzJs8qLjvd4Xgpue06DOdnLxAUHzM6+kDZ+HMZfJYuR+LtwGc2hgf5gsijff0ekUNXZiqATP7PF5mZxZ3Izoun1s4zG4LUMnvw2r+KqCKIw+3IQH03v+BCA9nMELNqbSf6tiWSrXJB3LAVGUcallcrw8V2t9EL4EhzJWrQUax5wLVMNS0+rUPA3k22Ncx4XXZS9o0MBH27Bo6BpNelZpS+/uh9KsNlY6bHCmJU9p8g7m3fVKn28H3KDYA5Pl/T8Z1ptDAVe0lXdQ2YoyyH2uyPIGHBZZIs2pDBS8R07+qN+E7Q==";
        let signature = Signature::new(token, timestamp, nonce, echostr);
        let result = msg_signature(&signature);
        let expected = "477715d11cdb4164915debcba66cb864d751f3e6";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_decode_msg() {
        let encode_ase_key = "jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C";
        let msg = "RypEvHKD8QQKFhvQ6QleEB4J58tiPdvo+rtK1I9qca6aM/wvqnLSV5zEPeusUiX5L5X/0lWfrf0QADHHhGd3QczcdCUpj911L3vg3W/sYYvuJTs3TUUkSUXxaccAS0qhxchrRYt66wiSpGLYL42aM6A8dTT+6k4aSknmPj48kzJs8qLjvd4Xgpue06DOdnLxAUHzM6+kDZ+HMZfJYuR+LtwGc2hgf5gsijff0ekUNXZiqATP7PF5mZxZ3Izoun1s4zG4LUMnvw2r+KqCKIw+3IQH03v+BCA9nMELNqbSf6tiWSrXJB3LAVGUcallcrw8V2t9EL4EhzJWrQUax5wLVMNS0+rUPA3k22Ncx4XXZS9o0MBH27Bo6BpNelZpS+/uh9KsNlY6bHCmJU9p8g7m3fVKn28H3KDYA5Pl/T8Z1ptDAVe0lXdQ2YoyyH2uyPIGHBZZIs2pDBS8R07+qN+E7Q==";
        let result = decode_msg(msg, encode_ase_key);
        let expected = Decrypt {
            msg: "<xml><ToUserName><![CDATA[wx5823bf96d3bd56c7]]></ToUserName>\n<FromUserName><![CDATA[mycreate]]></FromUserName>\n<CreateTime>1409659813</CreateTime>\n<MsgType><![CDATA[text]]></MsgType>\n<Content><![CDATA[hello]]></Content>\n<MsgId>4561255354251345929</MsgId>\n<AgentID>218</AgentID>
</xml>"
                .to_string(),
            receiveid: "wx5823bf96d3bd56c7".to_string(),
        };
        assert_eq!(result, Ok(expected));
    }
}

pub async fn access_token(id: &str, secret: &str) -> Result<AccessTokenRes, reqwest::Error> {
    let url = format!(
        "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}={}",
        id, secret
    );
    reqwest::get(url)
        .await
        .unwrap()
        .json::<AccessTokenRes>()
        .await
}

