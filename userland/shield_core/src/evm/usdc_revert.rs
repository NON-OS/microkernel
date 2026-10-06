//! Why a simulated USDC transfer reverted. Circle's contract reverts with a
//! message, not a custom error, and the messages below are the ones its
//! blacklist, pause and balance checks use.

/// The sentence for a USDC revert.
pub(super) fn why(revert: &[u8]) -> &'static str {
    let text = super::nox_revert::message(revert).unwrap_or_default();
    let has = |needle: &[u8]| text.windows(needle.len()).any(|w| w == needle);
    if has(b"blacklisted") {
        "USDC blocks the sender or the recipient."
    } else if has(b"paused") {
        "USDC transfers are paused."
    } else if has(b"exceeds balance") {
        "The USDC balance does not cover that."
    } else {
        "USDC refused this transfer when it was tried."
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::arithmetic_side_effects, clippy::integer_division)]
    use super::why;

    fn error(text: &str) -> Vec<u8> {
        let mut out = vec![0x08, 0xc3, 0x79, 0xa0];
        out.extend_from_slice(&[0u8; 31]);
        out.push(0x20);
        out.extend_from_slice(&[0u8; 24]);
        out.extend_from_slice(&(text.len() as u64).to_be_bytes());
        out.extend_from_slice(text.as_bytes());
        out.resize(out.len().div_ceil(32) * 32 + 4, 0);
        out
    }

    #[test]
    fn circles_revert_messages_read_as_sentences() {
        assert_eq!(
            why(&error("Blacklistable: account is blacklisted")),
            "USDC blocks the sender or the recipient."
        );
        assert_eq!(why(&error("Pausable: paused")), "USDC transfers are paused.");
        assert_eq!(
            why(&error("ERC20: transfer amount exceeds balance")),
            "The USDC balance does not cover that."
        );
        assert_eq!(why(&[]), "USDC refused this transfer when it was tried.");
    }
}
