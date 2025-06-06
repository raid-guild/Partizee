use k256::{
    elliptic_curve::sec1::ToEncodedPoint,
    elliptic_curve::{generic_array::GenericArray, Scalar},
    AffinePoint, ProjectivePoint, PublicKey, Secp256k1, SecretKey,
};

use hmac_sha512::HMAC;
use k256::elliptic_curve::group::Group;
use k256::elliptic_curve::sec1::FromEncodedPoint;
use k256::elliptic_curve::PrimeField;

#[allow(dead_code)]
pub const HARDENED_INDEXES_START: u32 = 0x80000000;
#[allow(dead_code)]
pub const MASTER_KEY_KEY: &[u8] = b"Bitcoin seed";
#[allow(dead_code)]
pub const HMAC_SHA_512: &str = "HmacSHA512";
#[allow(dead_code)]
pub struct ExtendedKey {
    private_key: Option<SecretKey>,
    public_key: PublicKey,
    chain_code: [u8; 32],
    depth: u8,
    child_number: u32,
}

pub struct Bip32;

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Bip32 {
    /// Generates a master key from seed
    pub fn generate_master_key(seed: &[u8]) -> Result<ExtendedKey, Box<dyn std::error::Error>> {
        let i = Self::hmac_sha512(MASTER_KEY_KEY, seed)?;

        let il = &i[..32];
        let ir = &i[32..];

        let private_key = SecretKey::from_slice(il)?;
        let public_key =
            PublicKey::from_sec1_bytes(&private_key.public_key().to_encoded_point(true).as_bytes());

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
    pub fn derive_child_key(
        parent: &ExtendedKey,
        child_index: u32,
    ) -> Result<ExtendedKey, Box<dyn std::error::Error>> {
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
            let mut data = parent
                .public_key()
                .to_encoded_point(true)
                .as_bytes()
                .to_vec();
            data.extend_from_slice(&child_index.to_be_bytes());
            data
        };

        let i = Self::hmac_sha512(&parent.chain_code(), &data)?;
        let il = &i[..32];
        let ir = &i[32..];

        // Interpret il as a scalar
        let il_scalar = Scalar::<Secp256k1>::from_repr(GenericArray::clone_from_slice(il));
        if il_scalar.is_none().into() {
            return Err("Invalid IL scalar".into());
        }
        let il_scalar: Scalar<Secp256k1> = il_scalar.unwrap();

        // Check if il_scalar is zero or >= curve order
        if il_scalar.is_zero().into() {
            return Err("Derived IL is zero".into());
        }

        let mut child_private_key = None;
        let child_public_key = if parent.has_private_key() {
            // Private parent: child private = (il + kpar) mod n
            let parent_key = parent.private_key().unwrap();
            let parent_scalar = Scalar::<Secp256k1>::from_repr(GenericArray::clone_from_slice(
                &parent_key.to_bytes(),
            ))
            .unwrap();
            let child_scalar: Scalar<Secp256k1> = il_scalar + parent_scalar;

            if child_scalar.is_zero().into() {
                return Err("Derived child private key is zero".into());
            }

            let child_secret = SecretKey::from_slice(&child_scalar.to_bytes())?;
            let child_pub = PublicKey::from_sec1_bytes(
                &child_secret.public_key().to_encoded_point(true).as_bytes(),
            )?;
            child_private_key = Some(child_secret);
            child_pub
        } else {
            // Public parent: child public = (il * G) + Kpar
            let il_point = ProjectivePoint::GENERATOR * il_scalar;

            let parent_point: AffinePoint =
                AffinePoint::from_encoded_point(&parent.public_key().to_encoded_point(true))
                    .expect("Invalid parent public key encoding")
                    .into();
            let child_point: ProjectivePoint = il_point + parent_point;
            if bool::from(child_point.is_identity()) {
                return Err("Derived child public key is infinity".into());
            }
            let encoded = AffinePoint::from(child_point).to_encoded_point(true);
            PublicKey::from_sec1_bytes(encoded.as_bytes())?
        };

        let mut chain_code = [0u8; 32];
        chain_code.copy_from_slice(ir);

        Ok(ExtendedKey {
            private_key: child_private_key,
            public_key: child_public_key,
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
        assert_eq!(hex::encode(private_key.to_bytes()).len(), 64);
    }

    #[test]
    fn test_hmac_sha512() {
        let key = b"000102030405060708090a0b0c0d0e0f";
        let data = b"000102030405060708090a0b0c0d0e0f";
        let hmac = Bip32::hmac_sha512(key, data).unwrap();
        assert_eq!(hex::encode(hmac).len(), 128);
    }
}
