use crate::utils::cryptography::bip32::{Bip32, ExtendedKey, HARDENED_INDEXES_START};
use crate::utils::cryptography::bip39::Bip39;
use bip32::{DerivationPath, XPrv};
use std::str::FromStr;
/// BIP44 constants
pub const PURPOSE: u32 = 44 | HARDENED_INDEXES_START;

/// Derives a BIP44 key from a master key (XPrv).
///
/// This uses the path: m/44'/coin_type'/account'/change/address_index
///
/// # Arguments
/// * `master_key` - The master extended private key (XPrv)
/// * `coin_type` - The coin type (see SLIP-44)
/// * `account` - The account index
/// * `change` - The change index (0 = external, 1 = internal)
/// * `address_index` - The address index
///
/// # Returns
/// * `XPrv` - The derived extended private key

pub fn derive_bip44_key(
    master_key: ExtendedKey,
    coin_type: u32,
    account: u32,
    change: u32,
    address_index: u32,
) -> Result<ExtendedKey, Box<dyn std::error::Error>> {
    // BIP44 path: m/44'/coin_type'/account'/change/address_index
    let path = [
        PURPOSE,
        coin_type | HARDENED_INDEXES_START,
        account | HARDENED_INDEXES_START,
        change,
        address_index,
    ];

    let mut key: ExtendedKey = master_key;
    for index in path.iter() {
        key = Bip32::derive_child_key(&key, *index)?;
    }
    if key.has_private_key() {
        Ok(key)
    } else {
        Err("Failed to derive child key".into())
    }
}

mod tests {
    use super::*;
    use bip39::Mnemonic;

    #[test]
    fn test_bip44_derivation() {
        let seed = Bip39::mnemonic_to_seed("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about", "password");

        let master_key = Bip32::generate_master_key(&seed).unwrap();

        let child_key = derive_bip44_key(master_key, 60, 0, 0, 0); // Ethereum example
        let priv_bytes = child_key.unwrap().private_key().unwrap().to_bytes();
        assert_eq!(priv_bytes.len(), 32);
    }
}
