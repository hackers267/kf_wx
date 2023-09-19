/// 客户账号管理
pub mod account;
/// 常量
mod constant;
/// 解密模块
pub mod decrypt;
// 加密模块
mod encrypt;
/// 客服消息
mod message;
/// 解析模块
mod parse;
/// 签名模块
pub mod signature;
/// 验证模块
mod verify;

use serde::Deserialize;

pub use message::*;
pub use parse::parse_callback_xml;
pub use verify::*;

#[derive(Debug, Deserialize)]
pub struct AccessTokenRes {
    pub errcode: i32,
    pub errmsg: String,
    pub access_token: String,
    pub expires_in: i32,
}

pub async fn access_token(id: &str, secret: &str) -> Result<AccessTokenRes, reqwest::Error> {
    let url = format!(
        "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
        id, secret
    );
    reqwest::get(url)
        .await
        .unwrap()
        .json::<AccessTokenRes>()
        .await
}
