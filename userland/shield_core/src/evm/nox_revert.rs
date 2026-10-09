//! Why a simulated NOX transfer reverted, from the revert data: the token's
//! custom errors by selector, and its one-letter `require` messages.

/// The reason a simulated transfer reverted, from its revert data.
pub(super) fn why(revert: &[u8]) -> &'static str {
    const PAUSE: [u8; 4] = [0xd9, 0x3c, 0x06, 0x65];
    const SHORT: [u8; 4] = [0xe4, 0x50, 0xd3, 0x8c];
    const RECEIVER: [u8; 4] = [0xec, 0x44, 0x2f, 0x05];
    const ERROR: [u8; 4] = [0x08, 0xc3, 0x79, 0xa0];
    match revert.get(..4) {
        Some(s) if s == PAUSE => "NOX transfers are paused on this network.",
        Some(s) if s == SHORT => "The NOX balance does not cover that.",
        Some(s) if s == RECEIVER => "The token refuses that recipient.",
        Some(s) if s == ERROR && message(revert) == Some(b"g") => {
            "The token blocks the sender or the recipient."
        }
        _ => "The token refused this transfer when it was tried.",
    }
}

/// The text of an `Error(string)` revert: past the selector, an offset word,
/// a length word, then the bytes, padded to a word.
pub(super) fn message(revert: &[u8]) -> Option<&[u8]> {
    let len_word = revert.get(36..68)?;
    let (high, low) = len_word.split_at(24);
    if high.iter().any(|b| *b != 0) {
        return None;
    }
    let len = usize::try_from(u64::from_be_bytes(low.try_into().ok()?)).ok()?;
    revert.get(68..68usize.checked_add(len)?)
}

#[cfg(kani)]
#[path = "nox_revert_kani.rs"]
mod nox_revert_kani;
