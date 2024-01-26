//
//  Copyright 2024 Ram Flux, LLC.
//

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, KeyInit,
};

use hkdf::Hkdf;
use k256::{ecdh::diffie_hellman, SecretKey};
use sha2::Sha256;

use pbkdf2::pbkdf2_hmac;

pub struct Pin {
    pub passwd: Vec<u8>,
    pub salt: Vec<u8>,
    pub wpriv_key: Vec<u8>,
}

impl Pin {
    pub fn new(passwd: Vec<u8>, salt: Vec<u8>, wpriv: Option<Vec<u8>>) -> Self {
        let wpriv_key = wpriv.unwrap_or_default();
        Self {
            passwd,
            salt,
            wpriv_key,
        }
    }

    pub fn derive_key(&self) -> Result<(Vec<u8>, Vec<u8>), crate::Error> {
        let mut dk = [0u8; 32];
        pbkdf2_hmac::<Sha256>(
            self.passwd.as_slice(),
            self.salt.as_slice(),
            100_000,
            &mut dk,
        );

        let secret_key = SecretKey::from_bytes(&dk.into()).unwrap();
        let public_key = secret_key.public_key();
        let shared_secret = diffie_hellman(secret_key.to_nonzero_scalar(), public_key.as_affine());
        let hkdf = Hkdf::<Sha256>::new(None, shared_secret.raw_secret_bytes());

        let mut okm = [0u8; 32 + 12];
        let _ = hkdf.expand(&[], &mut okm);
        let (aes_key, aes_iv) = okm.split_at(32);
        Ok((aes_key.to_vec(), aes_iv.to_vec()))
    }

    pub fn encrypt(&self, aes_key: &[u8], aes_iv: &[u8]) -> Result<Vec<u8>, crate::Error> {
        let cipher = Aes256Gcm::new(GenericArray::from_slice(aes_key));
        let nonce = GenericArray::from_slice(aes_iv);

        let ciphertext = cipher.encrypt(nonce, self.wpriv_key.as_ref())?;
        Ok(ciphertext)
    }

    pub fn decrypt(
        &self,
        aes_key: &[u8],
        aes_iv: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, crate::Error> {
        let cipher = Aes256Gcm::new(GenericArray::from_slice(aes_key));
        let nonce = GenericArray::from_slice(aes_iv);

        let decrypted_data = cipher.decrypt(nonce, ciphertext)?;

        Ok(decrypted_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn is_check() -> Result<(), crate::Error> {
        let pin = Pin::new(b"123456".to_vec(), b"some_salt".to_vec(), None);
        let (aes_key, aes_iv) = pin.derive_key()?;
        let ciphertext = hex::decode(
            "2fd8eb3e0551d48aeacfbf779d975c36a18c03ac46b2be80328a07ee5fecf0897293fca44fd79f",
        )
        .unwrap();
        let decrypted_data = pin.decrypt(&aes_key, &aes_iv, &ciphertext)?;

        let decrypted_str = std::str::from_utf8(&decrypted_data).unwrap();
        println!("Decrypted text: {}", decrypted_str);
        Ok(())
    }

    #[test]
    fn it_work() -> Result<(), crate::Error> {
        let wallet_priv_key = b"wallet private key data".to_vec();
        let pin = Pin::new(
            b"123456".to_vec(),
            b"some_salt".to_vec(),
            Some(wallet_priv_key.clone()),
        );
        let (aes_key, aes_iv) = pin.derive_key()?;

        let ciphertext = pin.encrypt(&aes_key, &aes_iv)?;

        let ciphertext_hex = hex::encode(ciphertext);
        println!("Ciphertext (Hex): {}", ciphertext_hex);

        let ciphertext = hex::decode(ciphertext_hex).unwrap();

        let decrypted_data = pin.decrypt(&aes_key, &aes_iv, &ciphertext)?;

        let decrypted_str = std::str::from_utf8(&decrypted_data).unwrap();

        println!("Decrypted text: {}", decrypted_str);

        //论证
        let aes_key_hex = hex::encode(&aes_key);
        let aes_iv_hex = hex::encode(&aes_iv);

        // 打印 aes_key 和 aes_iv 的十六进制表示
        println!("AES Key (Hex): {}", aes_key_hex);
        println!("AES IV (Hex): {}", aes_iv_hex);

        use base64::{engine::general_purpose::STANDARD, Engine as _};

        // 将 aes_key 和 aes_iv 转换为 Base64 字符串
        let aes_key_b64 = STANDARD.encode(&aes_key);
        let aes_iv_b64 = STANDARD.encode(&aes_iv);

        // 打印 aes_key 和 aes_iv 的 Base64 表示
        println!("AES Key (Base64): {}", aes_key_b64);
        println!("AES IV (Base64): {}", aes_iv_b64);

        assert_eq!(wallet_priv_key, decrypted_data, "Decryption failed");

        Ok(())
    }
}
