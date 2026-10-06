//! Reading a view key someone pasted or scanned. A wrong prefix, checksum, version, padding, or a
//! key word at or above p is refused, so one key has one spelling and nothing else reads as one.

use super::base32_read::decode;
use super::receive::ReceiveKey;
use super::view_key::{body, checksum, padded, ViewKind, FULL, INCOMING, VERSION};
use crate::error::WalletError;
use nonos_stark::field::P;
use zeroize::Zeroizing;

/// What a view key holds. The receiving key wipes itself, and there is no spend secret to hold.
pub(crate) struct ReadViewKey {
    pub kind: ViewKind,
    pub receive: ReceiveKey,
    pub spend_pk: [u64; 4],
    pub nk: Option<[u64; 4]>,
}

pub(crate) fn parse_view_key(text: &str) -> Result<ReadViewKey, WalletError> {
    let cleaned: Zeroizing<String> = Zeroizing::new(
        text.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase(),
    );
    let (kind, rest) = match (cleaned.strip_prefix(INCOMING), cleaned.strip_prefix(FULL)) {
        (Some(rest), _) => (ViewKind::Incoming, rest),
        (_, Some(rest)) => (ViewKind::Full, rest),
        _ => return Err(WalletError::Address),
    };
    let bytes = Zeroizing::new(decode(rest).ok_or(WalletError::Address)?);
    let n = body(kind);
    if bytes.len() != padded(kind) || bytes.first() != Some(&VERSION) {
        return Err(WalletError::Address);
    }
    let (key, tail) = bytes.split_at(n);
    let (check, pad) = tail.split_at(super::view_key::CHECK);
    if checksum(kind, key)[..] != *check || pad.iter().any(|b| *b != 0) {
        return Err(WalletError::Address);
    }
    let mut seed = Zeroizing::new([0u8; 32]);
    seed.copy_from_slice(key.get(1..33).ok_or(WalletError::Address)?);
    let spend_pk = words(key.get(33..65).ok_or(WalletError::Address)?)?;
    let nk = match kind {
        ViewKind::Incoming => None,
        ViewKind::Full => Some(words(key.get(65..97).ok_or(WalletError::Address)?)?),
    };
    Ok(ReadViewKey { kind, receive: ReceiveKey::from_x_wing_seed(seed), spend_pk, nk })
}

/// Four little endian field words, each below p.
fn words(bytes: &[u8]) -> Result<[u64; 4], WalletError> {
    let mut out = [0u64; 4];
    for (w, chunk) in out.iter_mut().zip(bytes.chunks_exact(8)) {
        let mut le = [0u8; 8];
        le.copy_from_slice(chunk);
        *w = u64::from_le_bytes(le);
        if *w >= P {
            return Err(WalletError::Address);
        }
    }
    Ok(out)
}
