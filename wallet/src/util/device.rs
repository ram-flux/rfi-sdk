//
//  Copyright 2024 Ram Flux, LLC.
//

use hkdf::Hkdf;
use rand_core::SeedableRng;
use sha2::Sha256;

#[derive(Debug, Clone)]
pub struct Device;

impl Device {
    pub fn generate_device_key(
        device_id: &[u8],
        account_public_key: &[u8],
        server_osrng: String,
    ) -> Result<(x25519_dalek::StaticSecret, x25519_dalek::PublicKey), crate::Error> {
        let salt_bytes = hex::decode(server_osrng)?;
        let info = [device_id, account_public_key].concat();

        let hkdf = Hkdf::<Sha256>::new(Some(&salt_bytes), &info);
        let mut okm = [0u8; 32];
        hkdf.expand(b"", &mut okm)?;

        let rng = rand_chacha::ChaChaRng::from_seed(okm);
        let prikey = x25519_dalek::StaticSecret::random_from_rng(rng);
        let pubkey = x25519_dalek::PublicKey::from(&prikey);

        Ok((prikey, pubkey))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::RngCore;

    #[test]
    fn test_generate_device() {
        let device_id = b"device-unique-id";
        let account_pk = b"account-public-key";

        //这是服务端随机数
        let mut server_osrng = rand_core::OsRng;
        let random_number = server_osrng.next_u64().to_le_bytes();
        let server_in = hex::encode(random_number.to_vec());

        let (prikey, pubkey) = Device::generate_device_key(device_id, account_pk, server_in)
            .map_err(|e| crate::Error::from(e))
            .unwrap();

        println!("prikey: {:#?}", prikey.as_bytes());
        println!("pubkey: {:#?}", pubkey);
    }

}
