/// 客户账号管理
pub mod account;
/// 常量
mod constant;
/// 客服消息
pub mod message;
use serde::Deserialize;

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

