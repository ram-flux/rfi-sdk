#[cfg(test)]
mod tests {

    // use chacha20poly1305::aead::generic_array::GenericArray;
    use chacha20poly1305::{
        aead::{Aead, KeyInit},
        ChaCha20Poly1305, Key, Nonce,
    };
    use hkdf::Hkdf;

    use sha2::Sha256;

    #[test]
    fn test_signature() -> Result<(), crate::Error> {
        // 假设的初始密钥材料
        let ikm = b"some initial key material here";

        // HKDF使用SHA-256来派生密钥
        let hkdf = Hkdf::<Sha256>::new(None, ikm);
        let mut chacha_key = [0u8; 32];
        let mut poly_key = [0u8; 32];
        hkdf.expand(b"CTX-ChaCha20", &mut chacha_key)?;
        hkdf.expand(b"CTX-Poly1305", &mut poly_key)?;

        // 创建ChaCha20加密器
        let key = Key::from_slice(&chacha_key);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = Nonce::from_slice(b"unique nonce");
        // 待加密的数据
        let plaintext = b"plaintext messagesdfsdfsdfsdfsdf sdfsdfsdf";

        // 加密
        let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())?;

        // 解密
        let decrypted_plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;

        // 验证解密后的数据是否与原始数据一致
        assert_eq!(&decrypted_plaintext[..], plaintext);


        Ok(())
    }
}
