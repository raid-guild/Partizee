use pbkdf2::pbkdf2_hmac;
use sha2::{Digest, Sha256, Sha512};
use unicode_normalization::UnicodeNormalization;

const ENGLISH_WORDLIST: &str = include_str!("wordlist.txt");
const PBKDF2_ROUNDS: u32 = 2048;
const MNEMONIC_SALT_PREFIX: &str = "mnemonic";

pub struct Bip39;
#[allow(dead_code)]
impl Bip39 {
    /// Generates a mnemonic phrase from given entropy
    pub fn generate_mnemonic(entropy: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        let entropy_bit_length = entropy.len() * 8;
        if entropy_bit_length % 32 != 0 || entropy_bit_length < 128 || entropy_bit_length > 256 {
            return Err(
                "Invalid entropy size, must be multiple of 32 and in range [128;256]".into(),
            );
        }

        let indexes = Self::get_word_indexes(entropy)?;
        let word_list: Vec<&str> = ENGLISH_WORDLIST.lines().collect();

        Ok(indexes
            .iter()
            .map(|&i| word_list[i as usize])
            .collect::<Vec<&str>>()
            .join(" "))
    }

    /// Convert mnemonic to seed using optional passphrase
    pub fn mnemonic_to_seed(mnemonic: &str, passphrase: &str) -> Vec<u8> {
        let normalized_mnemonic = mnemonic.nfkd().collect::<String>();
        let normalized_passphrase = passphrase.nfkd().collect::<String>();

        let salt = format!("{}{}", MNEMONIC_SALT_PREFIX, normalized_passphrase);
        let mut seed = vec![0u8; 64];

        pbkdf2_hmac::<Sha512>(
            normalized_mnemonic.as_bytes(),
            salt.as_bytes(),
            PBKDF2_ROUNDS,
            &mut seed,
        );

        seed
    }

    /// Validate a mnemonic phrase
    pub fn validate_mnemonic(mnemonic: &str) -> Result<(), Box<dyn std::error::Error>> {
        if mnemonic.is_empty() {
            return Err("Empty mnemonic".into());
        }

        let words: Vec<&str> = mnemonic.split_whitespace().collect();
        let word_count = words.len();

        if word_count % 3 != 0 || word_count < 12 || word_count > 24 {
            return Err(format!("Invalid word count: {}", word_count).into());
        }

        let indexes = Self::convert_words_to_indexes(&words)?;
        Self::check_checksum(&indexes)?;

        Ok(())
    }

    // Private helper functions
    fn get_word_indexes(entropy: &[u8]) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let mut entropy_with_checksum = entropy.to_vec();
        entropy_with_checksum.push(Self::compute_checksum(entropy));

        let mnemonic_length = ((entropy.len() * 8) + entropy.len() / 4) / 11;
        let mut indexes = Vec::with_capacity(mnemonic_length);

        for i in (0..mnemonic_length).map(|x| x * 11) {
            indexes.push(Self::get_next_11_bits(&entropy_with_checksum, i));
        }

        Ok(indexes)
    }

    fn compute_checksum(entropy: &[u8]) -> u8 {
        let mut hasher = Sha256::new();
        hasher.update(entropy);
        hasher.finalize()[0]
    }

    fn convert_words_to_indexes(words: &[&str]) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let word_list: Vec<&str> = ENGLISH_WORDLIST.lines().collect();

        words
            .iter()
            .map(|word| {
                word_list
                    .iter()
                    .position(|&w| w == *word)
                    .map(|i| i as u32)
                    .ok_or_else(|| format!("Invalid word in mnemonic: {}", word).into())
            })
            .collect()
    }

    fn check_checksum(indexes: &[u32]) -> Result<(), Box<dyn std::error::Error>> {
        let total_bits = indexes.len() * 11;
        let total_bytes = Self::get_byte_amount_from_bits(total_bits);
        let mut entropy_with_checksum = vec![0u8; total_bytes];
        Self::write_in_11_bit_chunks(&mut entropy_with_checksum, indexes);

        let checksum_bits = total_bits / 33;
        let entropy_bits = total_bits - checksum_bits;
        let entropy_bytes = Self::get_byte_amount_from_bits(entropy_bits);

        let entropy = &entropy_with_checksum[..entropy_bytes];
        let expected_checksum =
            Self::extract_bits(&entropy_with_checksum, entropy_bits, checksum_bits);

        let computed_checksum = Self::compute_checksum(entropy);
        let computed_checksum_bits = (computed_checksum as u32 & 0xFF) >> (8 - checksum_bits);

        if expected_checksum != computed_checksum_bits {
            return Err("Invalid checksum".into());
        }

        Ok(())
    }

    fn get_byte_amount_from_bits(bits: usize) -> usize {
        (bits + 7) / 8
    }

    fn get_next_11_bits(byte_array: &[u8], bit_offset: usize) -> u32 {
        let mut result = 0;
        let mut bits_collected = 0;
        let mut current_offset = bit_offset;

        while bits_collected < 11 {
            let byte_index = current_offset / 8;
            let bit_index = current_offset % 8;

            let current_bit = (byte_array[byte_index] >> (7 - bit_index)) & 1;
            result = (result << 1) | current_bit as u32;
            current_offset += 1;
            bits_collected += 1;
        }

        result
    }

    fn write_in_11_bit_chunks(byte_array: &mut [u8], values: &[u32]) {
        let mut bit_offset = 0;
        for &value in values {
            Self::write_11_bits(byte_array, bit_offset, value);
            bit_offset += 11;
        }
    }

    fn write_11_bits(array: &mut [u8], bit_offset: usize, value: u32) {
        for i in 0..11 {
            let bit = (value >> (11 - i - 1)) & 1;
            let total_bit_index = bit_offset + i;
            let byte_index = total_bit_index / 8;
            let bit_in_byte = 7 - (total_bit_index % 8);

            array[byte_index] |= (bit as u8) << bit_in_byte;
        }
    }

    fn extract_bits(data: &[u8], bit_offset: usize, num_bits: usize) -> u32 {
        let mut result = 0;
        for i in 0..num_bits {
            let total_bit_index = bit_offset + i;
            let byte_index = total_bit_index / 8;
            let bit_in_byte = 7 - (total_bit_index % 8);

            let bit = (data[byte_index] >> bit_in_byte) & 1;
            result = (result << 1) | bit as u32;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_mnemonic() {
        let entropy = [0u8; 16]; // 128 bits
        let mnemonic = Bip39::generate_mnemonic(&entropy).unwrap();
        assert!(Bip39::validate_mnemonic(&mnemonic).is_ok());
    }

    #[test]
    fn test_mnemonic_to_seed() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let passphrase = "";
        let seed = Bip39::mnemonic_to_seed(mnemonic, passphrase);
        assert_eq!(seed.len(), 64);
    }
}
