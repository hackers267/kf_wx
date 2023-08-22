/// 客户账号管理
pub mod account;
/// 常量
mod constant;
/// 客服消息
pub mod message;
use hex::encode;
use serde::Deserialize;
use sha1::{Digest, Sha1};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenRes {
    pub errcode: i32,
    pub errmsg: String,
    pub access_token: String,
    pub expires_in: i32,
}

pub fn msg_signature(token: &str, timestamp: &str, nonce: &str, echostr: &str) -> String {
    let mut arr = vec![token, timestamp, nonce, echostr];
    arr.sort();
    let str = arr.join("");
    let mut hasher = Sha1::new();
    hasher.update(str);
    let result = hasher.finalize();
    encode(result)
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
        let result = msg_signature(token, timestamp, nonce, echostr);
        let expected = "477715d11cdb4164915debcba66cb864d751f3e6";
        assert_eq!(result, expected);
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

