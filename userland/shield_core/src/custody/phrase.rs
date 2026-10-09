//! The recovery phrase, held as word list indices so no String copy of it is left in memory.
//! Parsing never guesses: an unknown word or a failed checksum is an error, not a nearest match.

use crate::error::CustodyError;
use nonos_hd::bip39::{word_index, words_to_entropy, MAX_WORDS};
use nonos_hd::ENGLISH_WORDLIST;

/// 12, 15, 18, 21 or 24 word indices, the BIP-39 lengths. A new phrase is always 24.
pub struct Phrase {
    indices: [u16; MAX_WORDS],
    len: usize,
}

impl Phrase {
    /// A phrase of `len` words, refused unless its length and checksum are BIP-39.
    pub(super) fn new(indices: [u16; MAX_WORDS], len: usize) -> Option<Phrase> {
        let phrase = Phrase { indices, len };
        words_to_entropy(phrase.indices()).map(|_| phrase)
    }

    pub(super) fn indices(&self) -> &[u16] {
        self.indices.get(..self.len).unwrap_or(&[])
    }

    /// The words, in order, for the screen that shows them.
    pub fn words(&self) -> Vec<String> {
        self.indices()
            .iter()
            .filter_map(|i| ENGLISH_WORDLIST.get(usize::from(*i)))
            .map(|word| (*word).to_string())
            .collect()
    }

    /// Parse a typed phrase. Words match case insensitively and the BIP39 checksum must hold.
    pub fn parse(words: &[String]) -> Result<Phrase, CustodyError> {
        if !matches!(words.len(), 12 | 15 | 18 | 21 | 24) {
            return Err(CustodyError::PhraseLength);
        }
        let mut indices = [0u16; MAX_WORDS];
        for (slot, word) in indices.iter_mut().zip(words.iter()) {
            *slot = word_index(word.trim().as_bytes()).ok_or(CustodyError::Mnemonic)?;
        }
        Phrase::new(indices, words.len()).ok_or(CustodyError::Mnemonic)
    }
}

impl Drop for Phrase {
    fn drop(&mut self) {
        for i in self.indices.iter_mut() {
            // SAFETY: a volatile write is a write the optimiser may not drop.
            unsafe { core::ptr::write_volatile(i, 0) };
        }
    }
}

impl core::fmt::Debug for Phrase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Phrase(redacted)")
    }
}
