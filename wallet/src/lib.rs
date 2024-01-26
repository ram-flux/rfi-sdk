//
//  Copyright 2024 Ram Flux, LLC.
//

mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use util::hdrf::Hdrf;
    use util::pin::Pin;

    #[test]
    fn test_hdrf() -> Result<(), Box<dyn std::error::Error>> {
        let hdrf = Hdrf::new("123456");
        let phrase = Hdrf::get_phrase();
        let (secret_key_hex, public_key_hex) = hdrf.get_pk_hex(&phrase).unwrap();
        println!("phrase:{}", phrase);
        println!("secret_key_hex:{}", secret_key_hex);
        println!("public_key_hex:{}", public_key_hex);

        let wallet_priv_key = secret_key_hex.as_bytes().to_vec();
        let pin = Pin::new(b"123456".to_vec(), b"some_salt".to_vec(), Some(wallet_priv_key));
        let (aes_key, aes_iv) = pin.derive_key()?;

        let ciphertext = pin.encrypt(&aes_key, &aes_iv)?;

        let ciphertext_hex = hex::encode(&ciphertext);
        println!("Ciphertext (Hex): {}", ciphertext_hex);

        let ciphertext = hex::decode(ciphertext_hex)?;

        let decrypted_data = pin.decrypt(&aes_key, &aes_iv, &ciphertext)?;

        let decrypted_str = std::str::from_utf8(&decrypted_data)
            .map_err(|e| format!("Decrypted data is not valid UTF-8: {}", e))?;

        println!("Decrypted text: {}", decrypted_str);
        Ok(())
    }
}
