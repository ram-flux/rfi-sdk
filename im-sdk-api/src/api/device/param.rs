#[derive(Debug, serde::Serialize)]
pub struct InitDeviceRes {
    pub(crate) device_id: u32,
}

impl InitDeviceRes {
    /// 初始化设备(tested)
    pub async fn init_device(
        account_pk: String,
        device_pk: String,
        proof: String,
        version: String,
        last_ip: String,
        platform: String,
        salt: String,
        name: String,
    ) -> Result<Self, crate::Error> {
        let device_id = crate::handler::device::init_device(
            account_pk, device_pk, proof, version, last_ip, platform, salt, name,
        )
        .await?;
        Ok(Self { device_id })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GetPhraseRes {
    pub(crate) phrase: String,
}

impl GetPhraseRes {
    pub(crate) fn get_phrase() -> Result<GetPhraseRes, crate::Error> {
        Ok(GetPhraseRes {
            phrase: wallet::get_phrase(),
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct GetPriKeyAndAccountRes {
    pub(crate) pin_secret: String,
    pub(crate) account: String,
}

impl GetPriKeyAndAccountRes {
    pub(crate) fn get_prikey_hex(
        phrase: &str,
        phrase_passwd: &str,
        pin: &str,
    ) -> Result<GetPriKeyAndAccountRes, crate::Error> {
        let (pin_secret, account) = wallet::get_pk_hex(phrase, phrase_passwd, pin)?;
        Ok(GetPriKeyAndAccountRes {
            pin_secret,
            account,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PinDecryptRes {
    pub(crate) decrypted: Vec<u8>,
}

impl PinDecryptRes {
    pub(crate) fn pin_decrypt(pk: String, passwd: Vec<u8>) -> Result<PinDecryptRes, crate::Error> {
        let decrypted = wallet::pin_decrypt(pk, passwd)?;
        Ok(PinDecryptRes { decrypted })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateTokenRes {
    pub(crate) token: String,
}

impl UpdateTokenRes {
    /// 初始化设备(tested)
    pub async fn update_token(device_id: u32, token: String) -> Result<Self, crate::Error> {
        let token = crate::handler::device::update_token(device_id).await?;
        Ok(Self { token })
    }
}
