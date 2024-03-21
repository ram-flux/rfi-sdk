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
) -> crate::response::Response<u32> {
    #[cfg(feature = "mock")]
    return Ok(454456).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::device::init_device(
            account_pk, device_pk, proof, version, last_ip, platform, salt, name,
        )
        .await
        .into()
    }
}

pub fn create_pin(_passwd: String) {}

// 获取助记词
pub fn get_phrase() -> String {
    wallet::Hdrf::get_phrase()
}

pub fn crypt_secret_key(secret_key_hex: String) -> Result<String, crate::Error> {
    let wallet_priv_key = secret_key_hex.as_bytes().to_vec();
    let pin = wallet::Pin::new(
        b"123456".to_vec(),
        b"some_salt".to_vec(),
        Some(wallet_priv_key),
    );
    let (aes_key, aes_iv) = pin.derive_key()?;

    let ciphertext = pin.encrypt(&aes_key, &aes_iv)?;

    Ok(hex::encode(ciphertext))
}

/// 授权
pub async fn warrant() -> crate::response::Response<String> {
    #[cfg(feature = "mock")]
    return Ok("server_pubkey".to_string()).into();
    #[cfg(not(feature = "mock"))]
    todo!()
}

/// 更新token(tested)
pub async fn update_token(device_id: u32, token: String) -> crate::response::Response<String> {
    #[cfg(feature = "mock")]
    return Ok("token".to_string()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::device::update_token(device_id).await.into()
    }
}

/// 删除设备(tested)
pub async fn del_device(device_id: u32) -> crate::response::Response<()> {
    #[cfg(feature = "mock")]
    return Ok(()).into();
    #[cfg(not(feature = "mock"))]
    {
        crate::handler::device::del_device(device_id).await.into()
    }
}

#[cfg(test)]
mod test {
    async fn init_db() {
        use crate::operator::sqlite::init::DbConnection;
        let pri_url = "sqlite://test_pri.db";
        let pub_url = "sqlite://test_pub.db";
        let res = DbConnection::init_user_database(pri_url.to_string()).await;
        println!("init_user_database res: {res:?}");
        let _ = DbConnection::init_pub_database(pub_url.to_string()).await;
    }

    #[tokio::test]
    async fn test_init_device() {
        init_db().await;

        let account_pk = "frefref".to_string();
        let device_pk = "asdsadasd".to_string();
        let proof = "efertr234234".to_string();
        let version = "22".to_string();
        let last_ip = "124.213.0.1".to_string();
        let platform = "macos".to_string();
        let salt = "salt".to_string();
        let name = "name".to_string();
        let res = crate::api::device::init_device(
            account_pk, device_pk, proof, version, last_ip, platform, salt, name,
        )
        .await;
        println!("res: {res:#?}");
    }

    #[tokio::test]
    async fn test_update_token() {
        crate::init_log();
        init_db().await;

        let device_id = 3393327105;
        let res = crate::api::device::update_token(device_id, "token".to_string()).await;
        println!("res: {res:#?}");
    }

    #[tokio::test]
    async fn test_del_device() {
        crate::init_log();
        init_db().await;

        let device_id = 3393327105;
        let res = crate::api::device::del_device(device_id).await;
        println!("res: {res:#?}");
    }
}
