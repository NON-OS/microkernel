//! The ABI words a deposit read sends and gets back, each decoded strictly.

use crate::error::NetError;

pub(super) fn call(selector: &[u8; 4], args: &[[u8; 32]]) -> Vec<u8> {
    let mut out = selector.to_vec();
    for arg in args {
        out.extend_from_slice(arg);
    }
    out
}

pub(super) fn asset_word(asset: u64) -> [u8; 32] {
    let mut w = [0u8; 32];
    let (_, low) = w.split_at_mut(24);
    low.copy_from_slice(&asset.to_be_bytes());
    w
}

pub(super) fn address_word(addr: &[u8; 20]) -> [u8; 32] {
    let mut w = [0u8; 32];
    let (_, low) = w.split_at_mut(12);
    low.copy_from_slice(addr);
    w
}

/// An ABI bool: zero or one, and nothing else.
pub(super) fn flag(word: [u8; 32]) -> Result<bool, NetError> {
    let (high, last) = word.split_at(31);
    match (high.iter().all(|b| *b == 0), last) {
        (true, [0]) => Ok(false),
        (true, [1]) => Ok(true),
        _ => Err(NetError::ReplyShape),
    }
}

/// An ABI uint256 as a u128, saturating. A cap beyond u128 is no cap at all.
pub(super) fn amount(word: [u8; 32]) -> u128 {
    let (high, low) = word.split_at(16);
    if high.iter().any(|b| *b != 0) {
        return u128::MAX;
    }
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(low);
    u128::from_be_bytes(bytes)
}
