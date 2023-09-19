use crate::decrypt::{decrypt_msg, DecryptErr};
use crate::signature::{msg_signature, Signature};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct WeiXinCallbackParam {
    timestamp: String,
    nonce: String,
    echostr: String,
    msg_signature: String,
}

/// 验证错误类型
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VerifyErr {
    Decrypt(DecryptErr),
    Signature,
}

impl From<DecryptErr> for VerifyErr {
    fn from(value: DecryptErr) -> Self {
        VerifyErr::Decrypt(value)
    }
}
/// 验证URL有效性
pub fn verify_url(
    callback_params: &WeiXinCallbackParam,
    token: &str,
    encoding_ase_key: &str,
) -> Result<String, VerifyErr> {
    let WeiXinCallbackParam {
        timestamp,
        nonce,
        echostr,
        msg_signature: old_signature,
    } = callback_params.clone();
    let signature = Signature::new(token, &timestamp, &nonce, &echostr);
    let new_signature = msg_signature(&signature);
    if old_signature == new_signature {
        Err(VerifyErr::Signature)
    } else {
        let msg = decrypt_msg(&echostr, encoding_ase_key)?.msg;
        Ok(msg)
    }
}
