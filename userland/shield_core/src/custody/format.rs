//! The vault file: magic, wrapped file key, sealed secret. Version 1 seals the seed, version 2 the
//! seed and the word indices, version 3 a private key. A length that does not account for every
//! part is refused, so a truncated vault cannot decode to a shorter ciphertext.

pub use super::kind::Kind;
use crate::error::CustodyError;

pub const MAGIC: &[u8; 8] = b"NOXSHLD1";
pub const MAGIC_WORDS: &[u8; 8] = b"NOXSHLD2";
pub const MAGIC_KEY: &[u8; 8] = b"NOXSHLD3";

/// Associated data for each seal, so a vault of another version or purpose does not open here.
pub const AAD: &[u8] = b"nox-shield/vault/v1";
pub const AAD_WORDS: &[u8] = b"nox-shield/vault/v2";
pub const AAD_KEY: &[u8] = b"nox-shield/vault/v3";

pub const SEALED_LEN: usize = 64 + nonos_seal::TAG_LEN;
/// The seed, the number of words, 24 little endian word index slots, and the tag.
pub const SEALED_WORDS_LEN: usize = 64 + 1 + 48 + nonos_seal::TAG_LEN;
pub const SEALED_KEY_LEN: usize = 32 + nonos_seal::TAG_LEN;

/// The vault as bytes: magic, the wrapped key length, the wrapped key, the sealed secret.
pub fn encode(kind: Kind, wrapped: &[u8], sealed: &[u8]) -> Result<Vec<u8>, CustodyError> {
    let len = u16::try_from(wrapped.len()).map_err(|_| CustodyError::VaultShape)?;
    let mut out =
        Vec::with_capacity(12usize.saturating_add(wrapped.len()).saturating_add(sealed.len()));
    out.extend_from_slice(match kind {
        Kind::Seed => MAGIC,
        Kind::SeedAndWords => MAGIC_WORDS,
        Kind::Key => MAGIC_KEY,
    });
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(wrapped);
    out.extend_from_slice(sealed);
    Ok(out)
}

/// Split a vault file back into its version, the wrapped key and the sealed secret.
pub fn decode(bytes: &[u8]) -> Result<(Kind, Vec<u8>, Vec<u8>), CustodyError> {
    let shape = || CustodyError::VaultShape;
    let (magic, rest) = bytes.split_at_checked(MAGIC.len()).ok_or_else(shape)?;
    let (kind, sealed_len) = match magic {
        m if m == MAGIC => (Kind::Seed, SEALED_LEN),
        m if m == MAGIC_WORDS => (Kind::SeedAndWords, SEALED_WORDS_LEN),
        m if m == MAGIC_KEY => (Kind::Key, SEALED_KEY_LEN),
        _ => return Err(shape()),
    };
    let (len_bytes, rest) = rest.split_at_checked(2).ok_or_else(shape)?;
    let mut len = [0u8; 2];
    len.copy_from_slice(len_bytes);
    let (wrapped, sealed) =
        rest.split_at_checked(usize::from(u16::from_le_bytes(len))).ok_or_else(shape)?;
    if sealed.len() != sealed_len {
        return Err(shape());
    }
    Ok((kind, wrapped.to_vec(), sealed.to_vec()))
}
