//
//  Copyright 2024 Ram Flux, LLC.
//

//we is coin type:369777 is RF as ram flux
//https://github.com/satoshilabs/slips/blob/master/slip-0044.md

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use secp256k1::ecdsa::Signature;
use secp256k1::hashes::{sha256, Hash};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

use tiny_hderive::bip32::ExtendedPrivKey;

pub struct Hdrf<'a> {
    pub passwd: &'a str,
}

impl<'a> Hdrf<'a> {
    pub fn new(passwd: &'a str) -> Self {
        Self { passwd }
    }

    fn get_coin_type(&self) -> String {
        let coin_type: u32 = 369777;
        format!("m/44'/{}'/0'/0/0", coin_type)
    }

    fn get_priv_key(&self, phrase: &str) -> Result<ExtendedPrivKey, crate::Error> {
        let mnemonic = Mnemonic::from_phrase(phrase, bip39::Language::English)
            .map_err(|e| crate::Error::Bip39(e.to_string()))?;
        let seed = Seed::new(&mnemonic, self.passwd);
        let ext_priv_key = ExtendedPrivKey::derive(seed.as_bytes(), self.get_coin_type().as_str())?;
        Ok(ext_priv_key)
    }

    pub fn get_phrase() -> String {
        // let words_type = MnemonicType::for_word_count(24).unwrap();
        let w24 = MnemonicType::Words24;
        let mnemonic = Mnemonic::new(w24, Language::English);
        mnemonic.phrase().to_string()
    }

    pub fn recover_priv_key(&self, phrase: &str) -> Result<SecretKey, crate::Error> {
        let ext_priv_key = self.get_priv_key(phrase)?;
        let secp_secret_key = SecretKey::from_slice(&ext_priv_key.secret())?;
        Ok(secp_secret_key)
    }

    pub fn get_pk(&self, phrase: &str) -> Result<(SecretKey, PublicKey), crate::Error> {
        let ext_priv_key = self.get_priv_key(phrase)?;
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&ext_priv_key.secret())?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Ok((secret_key, public_key))
    }

    pub fn get_pk_hex(&self, phrase: &str) -> Result<(String, String), crate::Error> {
        let (secret_key, public_key) = self.get_pk(phrase)?;
        let secret_key_hex = hex::encode(secret_key.secret_bytes());
        let public_key_hex = hex::encode(public_key.serialize());
        Ok((secret_key_hex, public_key_hex))
    }

    /**
     * get public account
     */
    pub fn get_pub_key_acc(public_key: PublicKey) -> Result<String, crate::Error> {
        let public_key_hex = hex::encode(public_key.serialize());
        // let base58_address = bs58::encode(public_key_hex).into_string();
        Ok(public_key_hex)
    }

    pub fn get_pri(secret_key: &str) -> Result<SecretKey, crate::Error> {
        let secret_key_vec = hex::decode(secret_key)?;
        let secret_key = SecretKey::from_slice(&secret_key_vec)?;
        Ok(secret_key)
    }

    pub fn sign_message(
        &self,
        message: &[u8],
        secret_key: &SecretKey,
    ) -> Result<Signature, crate::Error> {
        let digest = sha256::Hash::hash(message);
        let message = Message::from_digest(digest.to_byte_array());
        let secp = Secp256k1::new();
        let sig = secp.sign_ecdsa(&message, &secret_key);
        Ok(sig)
    }

    pub fn verify_signature(
        &self,
        message: &[u8],
        signature: &Signature,
        public_key: &PublicKey,
    ) -> Result<bool, crate::Error> {
        let digest = sha256::Hash::hash(message);
        let message = Message::from_digest(digest.to_byte_array());
        let secp = Secp256k1::new();
        let verify = secp.verify_ecdsa(&message, &signature, &public_key).is_ok();
        Ok(verify)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify_message() -> Result<(), crate::Error> {
        let hdrf = Hdrf::new("");
        let phrase = Hdrf::get_phrase();
        let secret_key = hdrf.recover_priv_key(&phrase)?;
        let (_, public_key) = hdrf.get_pk(&phrase)?;
        let message = b"Hello, world!";
        let signature = hdrf.sign_message(message, &secret_key)?;
        let verify_result = hdrf.verify_signature(message, &signature, &public_key)?;
        assert!(verify_result, "Signature verification failed.");

        Ok(())
    }

    #[test]
    fn test_invalid_signature() -> Result<(), crate::Error> {
        let hdrf = Hdrf::new("");
        //the test phrase two different
        let phrase1 = Hdrf::get_phrase();
        let secret_key1 = hdrf.recover_priv_key(&phrase1)?;

        let phrase2 = Hdrf::get_phrase();
        let (_, public_key2) = hdrf.get_pk(&phrase2)?;

        let message = b"Test message";

        let signature = hdrf.sign_message(message, &secret_key1)?;
        let verify_result = hdrf.verify_signature(message, &signature, &public_key2)?;

        // check is false
        assert!(
            !verify_result,
            "Signature verification unexpectedly succeeded."
        );

        Ok(())
    }

    #[test]
    fn test_hdrf() {
        let hdrf = Hdrf::new("123456");
        let phrase = Hdrf::get_phrase();
        let (secret_key_hex, public_key_hex) = hdrf.get_pk_hex(&phrase).unwrap();
        println!("phrase:{}", phrase);
        println!("secret_key_hex:{}", secret_key_hex);
        println!("public_key_hex:{}", public_key_hex);
    }

    #[test]
    fn test_hdrf_recover() -> Result<(), crate::Error> {
        let hdrf = Hdrf::new("123456");
        let phrase = "retreat clerk marine shoe dune evidence damage current inmate dose purchase search main vast curve latin crop glass melody sentence sheriff such fetch equal";
        let secret_key_hex = hdrf.recover_priv_key(phrase)?;
        println!("phrase:{}", phrase);
        println!("secret_key_hex:{:?}", secret_key_hex);
        Ok(())
    }

    #[test]
    fn test_hdrf_recover_pk() -> Result<(), crate::Error> {
        let hdrf = Hdrf::new("1234567");
        let phrase = "retreat clerk marine shoe dune evidence damage current inmate dose purchase search main vast curve latin crop glass melody sentence sheriff such fetch equal";
        let (secret_key, public_key) = hdrf.get_pk(phrase)?;
        println!("phrase:{}", phrase);
        println!("secret_key_hex:{:?}", secret_key);
        println!("public_key_hex:{:?}", public_key.to_string());
        let base58_address = bs58::encode(public_key.to_string().as_bytes()).into_string();
        println!("base58_address:{}", base58_address);
        Ok(())
    }
}
