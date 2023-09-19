use crate::constant::BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

fn format_url(token: &str) -> String {
    format!("{BASE_URL}/sync_msg?access_token={token}")
}

#[derive(Deserialize_repr, Serialize_repr, Clone, Debug, Copy)]
#[repr(u8)]
pub enum VoiceFormat {
    Amr = 0,
    Silk = 1,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMsg {
    pub cursor: Option<String>,
    pub token: Option<String>,
    pub limit: Option<i32>,
    pub voice_format: Option<VoiceFormat>,
    pub open_kfid: Option<String>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Copy)]
#[repr(u8)]
pub enum MoreMsg {
    Yes = 1,
    No = 0,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MsgRes {
    pub errcode: i32,
    pub errmsg: String,
    pub next_cursor: String,
    pub has_more: MoreMsg,
    pub msg_list: Vec<MsgItem>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum MsgOrigin {
    WeiXinCustomer = 3,
    System = 4,
    Kf = 5,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MsgItem {
    pub msgid: String,
    pub open_kfid: Option<String>,
    pub external_userid: Option<String>,
    pub send_time: u64,
    pub origin: MsgOrigin,
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
