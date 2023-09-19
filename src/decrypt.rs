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
use std::io::Error;

const G: GeneralPurpose = GeneralPurpose::new(
    &STANDARD,
    GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true),
);

type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Decrypt {
    pub msg: String,
    pub receiveid: String,
}

#[derive(Debug, PartialEq, Clone, Eq)]
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

/// 解密消息
pub fn decrypt_msg(msg: &str, encode_ase_key: &str) -> Result<Decrypt, DecryptErr> {
    let ase_key = decrypt_ase_key(encode_ase_key)?;
    let rand_msg = decrypt_orig_msg(msg, &ase_key)?;
    let rand_msg = slice_rand_msg(&rand_msg);
    let msg = extract_msg(&rand_msg)?;
    let receiveid = extract_receiveid(&rand_msg)?;
    Ok(Decrypt { msg, receiveid })
}

/// 计算msg长度
fn calc_msg_len(content: &[u8]) -> Result<u32, Error> {
    let mut msg_len = &content[0..4];
    Ok(msg_len.read_u32::<BigEndian>()? + 4)
}

/// 提取receiveid
fn extract_receiveid(rand_msg: &[u8]) -> Result<String, Error> {
    let content = &rand_msg[16..];
    let msg_len = calc_msg_len(content)?;
    let receiveid = &content[msg_len as usize..];
    let result = String::from_utf8_lossy(receiveid).to_string();
    Ok(result)
}

/// 提取消息
fn extract_msg(rand_msg: &[u8]) -> Result<String, Error> {
    let content = &rand_msg[16..];
    let msg_len = calc_msg_len(content)?;
    let msg = &content[4..msg_len as usize];
    let result = String::from_utf8_lossy(msg).to_string();
    Ok(result)
}

/// 截取随机消息
fn slice_rand_msg(rand_msg: &[u8]) -> Vec<u8> {
    let last = rand_msg[rand_msg.len() - 1];
    let rand_msg = &rand_msg[0..rand_msg.len() - last as usize];
    rand_msg.to_vec()
}

/// 解密原始消息
fn decrypt_orig_msg(msg: &str, ase_key: &[u8]) -> Result<Vec<u8>, DecryptErr> {
    let result = general_purpose::STANDARD.decode(msg)?;
    let iv = &ase_key[..16];
    let key = &ase_key[..32];
    let cipher = Aes256CbcDec::new_from_slices(key, iv)?;
    let mut buffer = vec![0u8; 1024];
    let result = cipher
        .decrypt_padded_b2b_mut::<NoPadding>(&result, &mut buffer)
        .map(|v| v.to_vec())?;
    Ok(result)
}

/// 解密ase_key
fn decrypt_ase_key(encode_ase_key: &str) -> Result<Vec<u8>, DecodeError> {
    let encode_ase_key = format!("{}=", encode_ase_key);
    G.decode(encode_ase_key)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_msg() {
        let encode_ase_key = "jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C";
        let msg = "RypEvHKD8QQKFhvQ6QleEB4J58tiPdvo+rtK1I9qca6aM/wvqnLSV5zEPeusUiX5L5X/0lWfrf0QADHHhGd3QczcdCUpj911L3vg3W/sYYvuJTs3TUUkSUXxaccAS0qhxchrRYt66wiSpGLYL42aM6A8dTT+6k4aSknmPj48kzJs8qLjvd4Xgpue06DOdnLxAUHzM6+kDZ+HMZfJYuR+LtwGc2hgf5gsijff0ekUNXZiqATP7PF5mZxZ3Izoun1s4zG4LUMnvw2r+KqCKIw+3IQH03v+BCA9nMELNqbSf6tiWSrXJB3LAVGUcallcrw8V2t9EL4EhzJWrQUax5wLVMNS0+rUPA3k22Ncx4XXZS9o0MBH27Bo6BpNelZpS+/uh9KsNlY6bHCmJU9p8g7m3fVKn28H3KDYA5Pl/T8Z1ptDAVe0lXdQ2YoyyH2uyPIGHBZZIs2pDBS8R07+qN+E7Q==";
        let result = decrypt_msg(msg, encode_ase_key);
        let expected = Decrypt {
            msg: "<xml><ToUserName><![CDATA[wx5823bf96d3bd56c7]]></ToUserName>\n<FromUserName><![CDATA[mycreate]]></FromUserName>\n<CreateTime>1409659813</CreateTime>\n<MsgType><![CDATA[text]]></MsgType>\n<Content><![CDATA[hello]]></Content>\n<MsgId>4561255354251345929</MsgId>\n<AgentID>218</AgentID>
</xml>"
                .to_string(),
            receiveid: "wx5823bf96d3bd56c7".to_string(),
        };
        assert_eq!(result, Ok(expected));
    }
}
