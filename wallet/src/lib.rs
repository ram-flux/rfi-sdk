//
//  Copyright 2024 Ram Flux, LLC.
//

mod error;
pub use error::Error;

mod util;
pub use util::{hdrf::Hdrf, pin::Pin};
// use secp256k1::PublicKey;

pub fn pin_encrypt(pk: Option<Vec<u8>>, passwd: Vec<u8>) -> Result<String, crate::Error> {
    let wallet_priv_key = pk;
    let salt = hex::encode(passwd.as_slice());
    let pin = Pin::new(passwd, salt.as_bytes().to_vec(), wallet_priv_key);
    let (aes_key, aes_iv) = pin.derive_key()?;
    let ciphertext = pin.encrypt(&aes_key, &aes_iv)?;
    let ciphertext_hex = hex::encode(ciphertext);
    Ok(ciphertext_hex)
}

pub fn pin_decrypt(pk: String, passwd: Vec<u8>) -> Result<Vec<u8>, crate::Error> {
    let salt = hex::encode(passwd.as_slice());
    let ciphertext = hex::decode(pk)?;
    let pin = Pin::new(passwd, salt.as_bytes().to_vec(), Some(ciphertext.clone()));
    let (aes_key, aes_iv) = pin.derive_key()?;
    let decrypted = pin.decrypt(&aes_key, &aes_iv, &ciphertext)?;
    Ok(decrypted)
}

pub fn get_phrase() -> String {
    Hdrf::get_phrase()
}

pub fn get_pk_hex(
    phrase: &str,
    phrase_passwd: &str,
    pin: &str,
) -> Result<(String, String), crate::Error> {
    let hdrf = Hdrf::new(phrase_passwd);
    let (secret_key, public_key) = hdrf.get_pk(&phrase)?;
    // println!("secret_key: {:?}", secret_key.secret_bytes().to_vec());
    let account = Hdrf::get_pub_key_acc(public_key)?;
    let pin_secret = pin_encrypt(
        Some(secret_key.secret_bytes().to_vec()),
        pin.as_bytes().to_vec(),
    )?;
    Ok((pin_secret, account))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phrase_pin_pk() {
        //Get private key and public key account
        let phrase = get_phrase();
        let (pin_secret, _public_key_hex) = get_pk_hex(&phrase, "123456", "123901").unwrap();
        // println!("pin_secret:{}", pin_secret);
        // println!("account:{}", public_key_hex);
        let pdec = pin_decrypt(pin_secret, b"123901".to_vec()).unwrap();
        println!("secret_key: {:?}", pdec);
    }

    #[test]
    fn test_pin_hd() -> Result<(), Box<dyn std::error::Error>> {
        let hdrf = Hdrf::new("123456");
        let phrase = Hdrf::get_phrase();
        let (secret_key_hex, _public_key_hex) = hdrf.get_pk_hex(&phrase).unwrap();
        println!("phrase:{}", phrase);
        // println!("secret_key_hex:{}", secret_key_hex);
        // println!("public_key_hex:{}", public_key_hex);

        let wallet_priv_key = secret_key_hex.as_bytes().to_vec();
        println!("wallet_priv_key:{:?}", wallet_priv_key);

        let pk = pin_encrypt(Some(wallet_priv_key.clone()), b"123456".to_vec())?;
        println!("pk:{}", pk);
        let pdec = pin_decrypt(pk, b"123456".to_vec())?;

        println!("Decrypted text: {:?}", pdec);

        assert_eq!(wallet_priv_key, pdec, "is ok");
        Ok(())
    }

    #[test]
    fn test_hdrf() -> Result<(), Box<dyn std::error::Error>> {
        let hdrf = Hdrf::new("123456");
        let phrase = Hdrf::get_phrase();
        let (secret_key_hex, public_key_hex) = hdrf.get_pk_hex(&phrase).unwrap();
        println!("phrase:{}", phrase);
        println!("secret_key_hex:{}", secret_key_hex);
        println!("public_key_hex:{}", public_key_hex);

        let wallet_priv_key = secret_key_hex.as_bytes().to_vec();
        let pin = Pin::new(
            b"123456".to_vec(),
            b"some_salt".to_vec(),
            Some(wallet_priv_key),
        );
        let (aes_key, aes_iv) = pin.derive_key()?;

        let ciphertext = pin.encrypt(&aes_key, &aes_iv)?;

        let ciphertext_hex = hex::encode(ciphertext);
        println!("Ciphertext (Hex): {}", ciphertext_hex);

        let ciphertext = hex::decode(ciphertext_hex)?;

        let decrypted_data = pin.decrypt(&aes_key, &aes_iv, &ciphertext)?;

        let decrypted_str = std::str::from_utf8(&decrypted_data)
            .map_err(|e| format!("Decrypted data is not valid UTF-8: {}", e))?;

        println!("Decrypted text: {}", decrypted_str);
        Ok(())
    }
}
