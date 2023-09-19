use quick_xml::de::from_str;
use quick_xml::DeError;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeiXinCallbackRes {
    pub to_user_name: String,
    pub create_time: String,
    pub msg_type: String,
    pub event: String,
    pub token: String,
    pub open_kf_id: String,
}
pub fn parse_callback_xml(xml: &str) -> Result<WeiXinCallbackRes, DeError> {
    from_str(xml)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_xml() {
        let str = "<xml><ToUserName><![CDATA[hello]]></ToUserName><CreateTime>1695102660</CreateTime><MsgType><![CDATA[event]]></MsgType><Event><![CDATA[kf_msg_or_event]]></Event><Token><![CDATA[world]]></Token><OpenKfId><![CDATA[zhangsan]]></OpenKfId></xml>";
        let result = parse_callback_xml(str).unwrap();
        assert_eq!(result.to_user_name, "hello");
        assert_eq!(result.create_time, "1695102660");
        assert_eq!(result.msg_type, "event");
        assert_eq!(result.event, "kf_msg_or_event");
        assert_eq!(result.token, "world");
        assert_eq!(result.open_kf_id, "zhangsan");
    }
}
