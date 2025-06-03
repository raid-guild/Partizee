use k256::{
    elliptic_curve::sec1::ToEncodedPoint,
    SecretKey, PublicKey,
    elliptic_curve::{generic_array::GenericArray, Field},
};
use pbkdf2::{pbkdf2_hmac};
use std::convert::TryFrom;
use hmac_sha512::{HMAC, Hash};

const HARDENED_INDEXES_START: u32 = 0x80000000;
const MASTER_KEY_KEY: &[u8] = b"Bitcoin seed";
const HMAC_SHA_512: &str = "HmacSHA512";

pub struct ExtendedKey {
    private_key: Option<SecretKey>,
    public_key: PublicKey,
    chain_code: [u8; 32],
    depth: u8,
    child_number: u32,
}

impl ExtendedKey {
    pub fn private_key(&self) -> Option<&SecretKey> {
        self.private_key.as_ref()
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn chain_code(&self) -> &[u8] {
        &self.chain_code
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }

    pub fn has_private_key(&self) -> bool {
        self.private_key.is_some()
    }
}

pub struct Bip32;

impl Bip32 {
    /// Generates a master key from seed
    pub fn generate_master_key(seed: &[u8]) -> Result<ExtendedKey, Box<dyn std::error::Error>> {
        let i = Self::hmac_sha512(MASTER_KEY_KEY, seed)?;
        
        let il = &i[..32];
        let ir = &i[32..];

        let private_key = SecretKey::from_slice(il)?;
        let public_key = PublicKey::from_sec1_bytes(
            &private_key
            .public_key()
            .to_encoded_point(true)
            .as_bytes());

        let mut chain_code = [0u8; 32];
        chain_code.copy_from_slice(ir);

        Ok(ExtendedKey {
            private_key: Some(private_key),
            public_key: public_key.unwrap(),
            chain_code,
            depth: 0,
            child_number: 0,
        })
    }

    /// Derives a child key from parent key
    pub fn derive_child_key(parent: &ExtendedKey, child_index: u32) -> Result<ExtendedKey, Box<dyn std::error::Error>> {
        let hardened = child_index & HARDENED_INDEXES_START != 0;
        
        let data = if hardened {
            if !parent.has_private_key() {
                return Err("Cannot derive hardened key from public key".into());
            }
            let mut data = vec![0u8];
            data.extend_from_slice(&parent.private_key().unwrap().to_bytes());
            data.extend_from_slice(&child_index.to_be_bytes());
            data
        } else {
            let mut data = parent.public_key().to_encoded_point(true).as_bytes().to_vec();
            data.extend_from_slice(&child_index.to_be_bytes());
            data
        };

        let i = Self::hmac_sha512(&parent.chain_code(), &data)?;
        let il = &i[..32];
        let ir = &i[32..];

        let child_key = if parent.has_private_key() {
            // Derive private child key
            let parent_key = parent.private_key().unwrap();
            let mut child_private_key = SecretKey::from_slice(il)?;
            
            // child_key = (IL + parent_private_key) mod n
            child_private_key = SecretKey::from_slice(
                &(child_private_key.to_bytes().as_slice()
                    .iter()
                    .zip(parent_key.to_bytes().as_slice())
                    .map(|(a, b)| a ^ b)
                    .collect::<Vec<u8>>())
            )?;
            
            Some(child_private_key)
        } else {
            None
        };

        let child_public_key = match &child_key {
            Some(pk) => PublicKey::from_sec1_bytes(&pk.public_key().to_encoded_point(true).as_bytes()),
            None => {
                // Derive public child key
                let il_point = PublicKey::from_sec1_bytes(il)?;
                let combined_point = il_point.to_projective() + parent.public_key().to_projective();
                let encoded = combined_point.to_affine().to_encoded_point(true);
                Ok(PublicKey::from_sec1_bytes(&encoded.as_bytes())?)
            }
        };

        let mut chain_code = [0u8; 32];
        chain_code.copy_from_slice(ir);

        Ok(ExtendedKey {
            private_key: child_key,
            public_key: child_public_key.unwrap(),
            chain_code,
            depth: parent.depth() + 1,
            child_number: child_index,
        })
    }

    fn hmac_sha512(key: &[u8], data: &[u8]) -> Result<[u8; 64], Box<dyn std::error::Error>> {
        let mac = HMAC::mac(data, key);
        Ok(mac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_master_key_generation() {
        let seed = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
        let master = Bip32::generate_master_key(&seed).unwrap();
        assert!(master.has_private_key());
        assert_eq!(master.depth(), 0);
    }

    #[test]
    fn test_child_key_derivation() {
        let seed = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
        let master = Bip32::generate_master_key(&seed).unwrap();
        
        // Derive normal child key
        let child = Bip32::derive_child_key(&master, 0).unwrap();
        assert!(child.has_private_key());
        assert_eq!(child.depth(), 1);

        // Derive hardened child key
        let hardened = Bip32::derive_child_key(&master, HARDENED_INDEXES_START).unwrap();
        assert!(hardened.has_private_key());
        assert_eq!(hardened.depth(), 1);
    }

    #[test]
    fn test_derive_private_key() {
        let seed = hex::decode("000102030405060708090a0b0c0d0e0f").unwrap();
        let master = Bip32::generate_master_key(&seed).unwrap();
        let child = Bip32::derive_child_key(&master, 0).unwrap();
        let private_key = child.private_key().unwrap();
        println!("{}", hex::encode(private_key.to_bytes()));
    }

    #[test]
    fn test_hmac_sha512() {
        let key = b"000102030405060708090a0b0c0d0e0f";
        let data = b"000102030405060708090a0b0c0d0e0f";
        let hmac = Bip32::hmac_sha512(key, data).unwrap();
        println!("{}", hex::encode(hmac));
    }

}
