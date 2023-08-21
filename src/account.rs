use crate::constant::BASE_URL;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub name: String,
    pub media_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AddRes {
    pub errcode: i32,
    pub errmsg: String,
    pub open_kfid: String,
}

fn format_url(path: &str) -> String {
    format!("{BASE_URL}/account/{path}")
}

/// 添加客服账号，并可设置客服名称和头像。目前一家企业最多可添加5000个客服账号。
pub async fn add(token: &str, account: &Account) -> Result<AddRes, reqwest::Error> {
    let client = Client::new();
    let path = format!("add?access_token={token}");
    let url = format_url(&path);
    client.post(url).json(account).send().await?.json().await
}

#[derive(Debug, Serialize)]
struct DelReq {
    kf_id: String,
}

impl DelReq {
    fn new(kf_id: &str) -> Self {
        Self {
            kf_id: kf_id.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SimpleRes {
    pub errocde: i32,
    pub errmsg: String,
}

/// 删除客服账号
pub async fn del(token: &str, kf_id: &str) -> Result<SimpleRes, reqwest::Error> {
    let client = Client::new();
    let path = format!("del?access_token={token}");
    let url = format_url(&path);
    let del_req = DelReq::new(kf_id);
    client
        .post(url)
        .json::<DelReq>(&del_req)
        .send()
        .await?
        .json()
        .await
}

#[derive(Debug, Serialize)]
pub struct UpdateAccount {
    pub name: String,
    pub open_kf_id: String,
    pub media_id: String,
}

/// 修改已有的客服账号，可修改客服名称和头像。
pub async fn update(token: &str, account: UpdateAccount) -> Result<SimpleRes, reqwest::Error> {
    let client = Client::new();
    let path = format!("update?access_token={token}");
    let url = format_url(&path);
    client
        .post(url)
        .json::<UpdateAccount>(&account)
        .send()
        .await?
        .json()
        .await
}

#[derive(Debug, Serialize)]
pub struct Page {
    pub offset: usize,
    pub limit: usize,
}

#[derive(Debug, Deserialize)]
pub struct ListItemRes {
    pub open_kfid: String,
    pub name: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize)]
pub struct ListRes {
    pub errcode: i32,
    pub errmsg: String,
    pub account_list: Vec<ListItemRes>,
}

/// 获取客服账号列表，包括所有的客服账号的客服ID、名称和头像
pub async fn list(token: &str, page: &Page) -> Result<ListRes, reqwest::Error> {
    let client = Client::new();
    let path = format!("list?access_token={token}");
    let url = format_url(&path);
    client.post(url).json(page).send().await?.json().await
}

#[derive(Debug, Serialize)]
pub struct AccountLinkReq {
    pub open_kfid: String,
    pub scene: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountLinkRes {
    pub errcode: i32,
    pub errmsg: String,
    pub url: String,
}

/// 企业可通过此接口获取带有不同参数的客服链接，不同客服账号对应不同的客服链接。获取后，企业可将链接嵌入到网页等场景中，微信用户点击链接即可向对应的客服账号发起咨询。企业可依据参数来识别用户的咨询来源等。
pub async fn link(token: &str, account: &AccountLinkReq) -> Result<AccountLinkRes, reqwest::Error> {
    let client = Client::new();
    let path = format!("add_contact_way?access_token={token}");
    let url = format_url(&path);
    client.post(url).json(account).send().await?.json().await
}
