use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize)]
struct TextMsg {
    menu_id: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImageMsg {
    media_id: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Message {
    /// 文本消息
    Text { menu_id: String, content: String },
    /// 图片消息
    Image { media_id: String },
    /// 语音消息
    Voice { media_id: String },
    /// 视频消息
    Video { media_id: String },
    /// 文件消息
    File { media_id: String },
    /// 位置消息
    Location {
        latitude: f32,
        longitude: f32,
        name: String,
        address: String,
    },
    /// 链接消息
    Link {
        title: String,
        desc: String,
        url: String,
        pic_url: String,
    },
    /// 名片消息
    BusinessCard { userid: String },
    /// 小程序消息
    Miniprogram {
        title: String,
        appid: String,
        pagepath: String,
        thumb_media_id: String,
    },
    /// TODO: 菜单消息
    Msgmenu {},
    /// 视频号产品消息
    ChannelsShopProduct {
        product_id: String,
        head_image: String,
        title: String,
        sales_price: String,
        shop_nickname: String,
        shop_head_image: String,
    },
    ChannelsShopOrder {
        order_id: String,
        product_titles: String,
        price_wording: String,
        state: String,
        image_url: String,
        shop_nickname: String,
    },
}

impl Message {}

/// 消息类型判断
impl Message {
    /// 判断消息是否为文本消息
    pub fn is_text(&self) -> bool {
        matches!(self, Message::Text { .. })
    }
    /// 判断消息是否为图片消息
    pub fn is_image(&self) -> bool {
        matches!(self, Message::Image { .. })
    }
    /// 判断消息是否为语音消息

    pub fn is_voice(&self) -> bool {
        matches!(self, Message::Voice { .. })
    }
    /// 判断消息是否为视频消息
    pub fn is_video(&self) -> bool {
        matches!(self, Message::Video { .. })
    }
    /// 判断消息是否为文件消息
    pub fn is_file(&self) -> bool {
        matches!(self, Message::File { .. })
    }
    /// 判断消息是否为位置消息
    pub fn is_location(&self) -> bool {
        matches!(self, Message::Location { .. })
    }
    /// 判断消息是否为链接消息
    pub fn is_link(&self) -> bool {
        matches!(self, Message::Link { .. })
    }
    /// 判断消息是否名片消息
    pub fn is_business_card(&self) -> bool {
        matches!(self, Message::BusinessCard { .. })
    }
    /// 判断消息是否小程序消息
    pub fn is_miniprogram(&self) -> bool {
        matches!(self, Message::Miniprogram { .. })
    }
    /// 判断消息是否视频号产品消息
    pub fn is_channels_shop_product(&self) -> bool {
        matches!(self, Message::ChannelsShopProduct { .. })
    }
    /// 判断消息是否视频号订单消息
    pub(crate) fn is_channels_shop_order(&self) -> bool {
        matches!(self, Message::ChannelsShopOrder { .. })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MsgItem {
    pub msgid: String,
    pub open_kfid: Option<String>,
    pub external_userid: Option<String>,
    pub send_time: u64,
    pub origin: MsgOrigin,
    pub servicer_userid: Option<String>,
    pub msgtype: String,
    #[serde(flatten)]
    pub message: Message,
}
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum MsgOrigin {
    WeiXinCustomer = 3,
    System = 4,
    Kf = 5,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn test_text_msg() {
        let str = r#"
            "msgtype": "text",
            "text": {
                "menu_id": "101",
                "content": "hello world"
            }
        "#;
        let data = gen_data(str);
        let msg: MsgItem = from_str(&data).unwrap();
        assert!(msg.message.is_text())
    }
    #[test]
    fn test_image_msg() {
        let str = r#"
            "msgtype": "image",
            "image": {
                "media_id": "hello image"
            }
        "#;
        let data = gen_data(str);
        let msg: MsgItem = from_str(&data).unwrap();
        assert!(msg.message.is_image());
    }

    #[test]
    fn test_voice_msg() {
        let str = r#"
            "msgtype": "voice",
            "voice": {
                "media_id": "hello voice"
            }
        "#;
        let data = gen_data(str);
        let msg: MsgItem = from_str(&data).unwrap();
        assert!(msg.message.is_voice());
    }

    #[test]
    fn test_video_msg() {
        let str = r#"
            "msgtype": "video",
            "video": {
                "media_id": "hello vedio"
            }
        "#;
        let data = gen_data(str);
        let msg: MsgItem = from_str(&data).unwrap();
        assert!(msg.message.is_video())
    }

    #[test]
    fn test_file_msg() {
        let str = r#"
            "msgtype": "file",
            "file": {
                "media_id": "hello file"
            }
        "#;
        let data = gen_data(str);
        let msg: MsgItem = from_str(&data).unwrap();
        assert!(msg.message.is_file());
    }

    #[test]
    fn test_location_msg() {
        let str = r#"
            "msgtype" : "location",
            "location" : {
                 "latitude": 23.106021881103501,
                 "longitude": 113.320503234863,
                 "name": "广州国际媒体港(广州市海珠区)",
                 "address": "广东省广州市海珠区滨江东路"
            }
        "#;
        let data = gen_data(str);
        let msg: MsgItem = from_str(&data).unwrap();
        assert!(msg.message.is_location());
    }

    #[test]
    fn test_link_msg() {
        let str = r#"
            "msgtype" : "link",
            "link" : {
                 "title": "TITLE",
                 "desc": "DESC",
                 "url": "URL",
                 "pic_url": "PIC_URL"
            }
        "#;
        let data = gen_data(str);
        let msg: MsgItem = from_str(&data).unwrap();
        assert!(msg.message.is_link());
    }

    #[test]
    fn test_business_card_msg() {
        let str = r#"
           "msgtype" : "business_card",
           "business_card" : {
                "userid": "USERID"
           }
        "#;
        let msg = parse_msg_item(str).unwrap();
        assert!(msg.message.is_business_card());
    }

    #[test]
    fn test_miniprogram_msg() {
        let str = r#"
           "msgtype" : "miniprogram",
           "miniprogram" : {
                "title": "TITLE",
                "appid": "APPID",
                "pagepath": "PAGE_PATH",
                "thumb_media_id": "THUMB_MEDIA_ID"
           }
        "#;
        let msg = parse_msg_item(str).unwrap();
        assert!(msg.message.is_miniprogram())
    }

    #[test]
    fn test_channels_shop_product_msg() {
        let str = r#"
           "msgtype" : "channels_shop_product",
           "channels_shop_product" : {
                "product_id": "PRODUCT_ID",
                "head_image": "PRODUCT_IMAGE_URL",
                "title": "TITLE",
                "sales_price": "SALES_PRICE",
                "shop_nickname": "SHOP_NICKNAME",
                "shop_head_image": "SHOP_HEAD_IMAGE"
           }
        "#;
        let msg = parse_msg_item(str).unwrap();
        assert!(msg.message.is_channels_shop_product())
    }

    #[test]
    fn test_channels_shop_order_msg() {
        let str = r#"
           "msgtype" : "channels_shop_order",
           "channels_shop_order" : {
                "order_id": "ORDER_ID",
                "product_titles":"PRODUCT_TITLES",
                "price_wording":"PRICE_WORDING",
                "state":"STATE",
                "image_url":"IMAGE_URL",
                "shop_nickname":"SHOP_NICKNAME"
           }
        "#;
        let msg = parse_msg_item(str).unwrap();
        assert!(msg.message.is_channels_shop_order())
    }

    fn parse_msg_item(str: &str) -> serde_json::Result<MsgItem> {
        let data = gen_data(str);
        from_str(&data)
    }

    fn gen_data(str: &str) -> String {
        const COMMON_STR: &str = r#"{
            "msgid": "from_msgid_4622416642169452483",
            "open_kfid": "wkAJ2GCAAASSm4_FhToWMFea0xAFfd3Q",
            "external_userid": "wmAJ2GCAAAme1XQRC-NI-q0_ZM9ukoAw",
            "send_time": 1615478585,
            "origin": 3,
			"servicer_userid": "Zhangsan",
    "#;
        format!("{}{}{}", COMMON_STR, str, "}")
    }
}
