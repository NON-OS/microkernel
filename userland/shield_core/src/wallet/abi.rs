//! The few pieces of the Solidity ABI the wallet writes. Offsets count from the enclosing tuple.

/// A number as one ABI word.
pub(crate) fn word(n: u128) -> [u8; 32] {
    let mut out = [0u8; 32];
    let (_, low) = out.split_at_mut(16);
    low.copy_from_slice(&n.to_be_bytes());
    out
}

fn len_word(n: usize) -> [u8; 32] {
    word(u128::try_from(n).unwrap_or(u128::MAX))
}

/// `bytes`: its length, then the bytes padded to a whole word.
pub(crate) fn bytes(b: &[u8]) -> Vec<u8> {
    let mut out = len_word(b.len()).to_vec();
    out.extend_from_slice(b);
    out.resize(out.len().div_ceil(32).saturating_mul(32), 0);
    out
}

/// `uint256[]`: its length, then the words.
pub(crate) fn words(ws: &[[u8; 32]]) -> Vec<u8> {
    let mut out = len_word(ws.len()).to_vec();
    ws.iter().for_each(|w| out.extend_from_slice(w));
    out
}

/// `bytes[]`: its length, an offset per element, then each element.
pub(crate) fn bytes_array(items: &[Vec<u8>]) -> Vec<u8> {
    tuple_of(&items.iter().map(|b| bytes(b)).collect::<Vec<_>>(), Some(items.len()))
}

/// A head of offsets, one per dynamic part, then the parts, with a length word first for arrays.
pub(crate) fn tuple_of(parts: &[Vec<u8>], length: Option<usize>) -> Vec<u8> {
    let mut head = length.map(|n| len_word(n).to_vec()).unwrap_or_default();
    let mut at = parts.len().saturating_mul(32);
    for part in parts {
        head.extend_from_slice(&len_word(at));
        at = at.saturating_add(part.len());
    }
    parts.iter().for_each(|p| head.extend_from_slice(p));
    head
}
