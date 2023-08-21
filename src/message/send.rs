use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::constant::BASE_URL;
/// 请求地址
fn format_url(token: &str) -> String {
    format!("{BASE_URL}/send_msg?access_token={token}")
}

#[derive(Debug, Deserialize)]
pub struct MessageRes {
    pub errcode: i32,
    pub errmsg: String,
    pub msgid: String,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub touser: String,
    pub open_kfid: String,
    pub msgid: Option<String>,
    pub msgtype: MsgType,
}

#[derive(Debug, Serialize)]
pub struct Link {
    pub title: String,
    pub desc: Option<String>,
    pub url: String,
    pub thumb_media_id: String,
}

#[derive(Debug, Serialize)]
pub struct MiniProgram {
    pub appid: String,
    pub title: Option<String>,
    pub thumb_media_id: String,
    pub pagepath: String,
}

#[derive(Debug, Serialize)]
pub struct Location {
    pub name: Option<String>,
    pub address: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize)]
pub enum MsgType {
    Text(String),
    Image(String),
    Voice(String),
    Video(String),
    File(String),
    Link(Link),
    MiniProgram(MiniProgram),
    Menu(Menu),
    Location(Location),
}

#[derive(Debug, Serialize)]
pub struct Menu {
    pub head_content: Option<String>,
    pub list: Vec<MenuItem>,
    pub tail_content: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum MenuItem {
    Click(String, String),
    View(String, String),
    MiniProgram(String, String, String),
    Text(String, i32),
}

/// 发送消息
pub async fn send(token: &str, message: &Message) -> Result<MessageRes, reqwest::Error> {
    let url = format_url(token);
    let client = Client::new();
    client
        .post(url)
        .json(message)
        .send()
        .await?
        .json::<MessageRes>()
        .await
}
