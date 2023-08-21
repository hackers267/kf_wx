use serde::{Deserialize, Serialize};

use crate::constant::BASE_URL;
fn format_url(token: &str) -> String {
    format!("{BASE_URL}/send_msg_on_evnet?access=token={}", token)
}

#[derive(Debug, Deserialize)]
pub struct WelcomeRes {
    errcode: i32,
    errmsg: String,
    msgid: String,
}

#[derive(Debug, Serialize)]
pub enum MsgType {
    Text(String),
    Menu(Vec<MenuItem>),
}

#[derive(Debug, Serialize)]
pub enum MenuItem {
    Click(String, String),
    View(String, String),
    Miniprogram(String, String, String),
    Text(String, String),
}

#[derive(Debug, Serialize)]
pub struct Welcome {
    pub code: String,
    pub msgid: String,
    msgtype: MsgType,
}

pub async fn send_welcome(token: &str, welcome: &Welcome) -> Result<WelcomeRes, reqwest::Error> {
    let url = format_url(token);
    let client = reqwest::Client::new();
    client
        .post(url)
        .json(welcome)
        .send()
        .await?
        .json::<WelcomeRes>()
        .await
}
