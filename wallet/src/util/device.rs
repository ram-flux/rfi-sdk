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
        println!("server_in: {:#?}", server_in);

        let (prikey, pubkey) = Device::generate_device_key(device_id, account_pk, server_in)
            .map_err(|e| crate::Error::from(e))
            .unwrap();
        //9a71b0558ed0abf9
        //3447a0b8093193b7a0330b632c8d4e7566ebdefcfcaf44529a97d3be5d89615a
        //edb4fe7a1d5aae37dfc37b9e575fcd36922a02a0e3ffcb79ba85268a5fb5b120

        // server_in: 2071e22e96ea8409
        // prikey: 58f13e43e096bf261e02d0db27e72f0920f13736586e204acba3aec377ee4020
        // pubkey: 2df994be6eb07a574a1a859032e6d395afc1de14dbac5c235ac3d153c868001b

        println!("prikey: {:?}", hex::encode(prikey.to_bytes()));
        println!("pubkey: {:#?}", hex::encode(pubkey.as_bytes()));
    }
}
