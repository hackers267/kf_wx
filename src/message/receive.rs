use crate::constant::BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};

fn format_url(token: &str) -> String {
    format!("{BASE_URL}/sync_msg?access_token={token}")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMsg {
    pub cursor: Option<String>,
    pub token: Option<String>,
    pub limit: Option<i32>,
    // TODO: 修改为枚举类型
    pub voice_format: Option<i32>,
    pub open_kfid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MsgRes {
    pub errcode: i32,
    pub errmsg: String,
    pub next_cursor: String,
    // TODO: 修改为枚举类型
    pub has_more: i32,
    pub msg_list: Vec<MsgItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MsgItem {
    pub msgid: String,
    pub open_kfid: Option<String>,
    pub external_userid: Option<String>,
    pub send_time: u64,
    pub origin: u32,
    pub servicer_userid: Option<String>,
}
/// 接收消息
pub async fn sync_msg(token: &str, msg: &SyncMsg) -> reqwest::Result<MsgRes> {
    let url = format_url(token);
    let client = Client::new();
    client
        .post(url)
        .json(msg)
        .send()
        .await?
        .json::<MsgRes>()
        .await
}
