//! Drawing a new 24-word phrase, and turning a phrase into a seed, through nonos_hd BIP-39.
//! Every intermediate buffer is wiped before returning.

use super::phrase::Phrase;
use super::secret::Seed;
use crate::entropy::fill;
use crate::error::CustodyError;
use nonos_hd::bip39::{entropy_to_words, seed_from_words, MAX_WORDS};
use nonos_hd::wipe;

/// Draw a new 24 word phrase from the platform CSPRNG.
pub fn generate_phrase() -> Result<Phrase, CustodyError> {
    let mut entropy = [0u8; 32];
    fill(&mut entropy)?;
    let mut indices = [0u16; MAX_WORDS];
    let n = entropy_to_words(&entropy, &mut indices);
    wipe(&mut entropy);
    match n {
        Some(MAX_WORDS) => Phrase::new(indices, MAX_WORDS).ok_or(CustodyError::Mnemonic),
        _ => Err(CustodyError::Mnemonic),
    }
}

/// Derive the seed a phrase stands for. The BIP-39 passphrase is empty: a second secret to
/// remember, on a phone, loses more wallets than it saves.
pub fn phrase_to_seed(phrase: &Phrase) -> Result<Seed, CustodyError> {
    let mut bytes = [0u8; 64];
    if !seed_from_words(phrase.indices(), &[], &mut bytes) {
        wipe(&mut bytes);
        return Err(CustodyError::Mnemonic);
    }
    let seed = Seed::new(bytes);
    wipe(&mut bytes);
    Ok(seed)
}
