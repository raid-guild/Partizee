use bip32::{XPrv, DerivationPath};
use std::str::FromStr;
/// BIP44 constants
pub const PURPOSE: u32 = 44;

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
    master_key: &XPrv,
    coin_type: u32,
    account: u32,
    change: u32,
    address_index: u32,
) -> XPrv {
    // Hardened indices: add 0x80000000 or use apostrophe in path string
    let path = DerivationPath::from_str(&format!(
        "m/{}'/{}'/{}'/{}/{}",
        PURPOSE, coin_type, account, change, address_index
    )).unwrap();

    let mut key = master_key.clone();
    for child_number in path.into_iter() {
        key = key.derive_child(child_number).unwrap();
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip39::{Mnemonic};

    #[test]
    fn test_bip44_derivation() {
        let mnemonic = Mnemonic::parse("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap();
        let seed = mnemonic.to_seed_normalized("password");
        let master_key = XPrv::new(seed).unwrap();

        let child_key = derive_bip44_key(&master_key, 60, 0, 0, 0); // Ethereum example
        let priv_bytes = child_key.private_key().to_bytes();
        assert_eq!(priv_bytes.len(), 32);
    }
}
