//! The two spellings an EVM wallet speaks: a 20-byte address as `0x` hex, and
//! transaction data as `0x` hex for the user to paste into their own wallet.

use crate::error::WalletError;

/// A `0x` address of forty hex digits, in either case. The checksum casing is not
/// enforced, because a mistyped digit fails the pool's own allowlist before any
/// money moves, and the refusal names it.
pub(crate) fn parse_evm_address(text: &str) -> Result<[u8; 20], WalletError> {
    let digits = text.trim().strip_prefix("0x").ok_or(WalletError::Address)?;
    // from_str_radix takes a leading sign, so each digit is checked first.
    if digits.len() != 40 || !digits.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(WalletError::Address);
    }
    let mut out = [0u8; 20];
    for (byte, pair) in out.iter_mut().zip(digits.as_bytes().chunks_exact(2)) {
        let pair = core::str::from_utf8(pair).map_err(|_| WalletError::Address)?;
        *byte = u8::from_str_radix(pair, 16).map_err(|_| WalletError::Address)?;
    }
    Ok(out)
}

/// Bytes as `0x` lowercase hex.
pub(crate) fn to_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::from("0x");
    for b in bytes {
        for nibble in [b >> 4, b & 0x0f] {
            out.extend(DIGITS.get(usize::from(nibble)).map(|d| char::from(*d)));
        }
    }
    out
}
