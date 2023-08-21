use reqwest::Client;
use serde::Serialize;

use crate::{account::SimpleRes, constant::BASE_URL};
/// 请求地址
fn format_url(token: &str) -> String {
    format!("{BASE_URL}/recall_msg?access_token={token}")
}

#[derive(Debug, Serialize)]
pub struct Message {
    msgid: String,
    open_kfid: String,
}

/// 撤回消息
pub async fn recall_msg(token: &str, message: &Message) -> Result<SimpleRes, reqwest::Error> {
    let url = format_url(token);
    let client = Client::new();
    client
        .post(url)
        .json(message)
        .send()
        .await?
        .json::<SimpleRes>()
        .await
}
