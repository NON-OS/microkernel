//! The `0x` hex a reply spells its bytes and numbers in.

/// A `0x` prefixed 32 byte word.
pub(super) fn bytes32(hex: &str) -> Option<[u8; 32]> {
    let bytes = hex_bytes(hex)?;
    <[u8; 32]>::try_from(bytes.as_slice()).ok()
}

/// The bytes a `0x` prefixed hex string stands for, empty for `0x`.
pub(crate) fn hex_bytes(hex: &str) -> Option<Vec<u8>> {
    let body = hex.strip_prefix("0x")?;
    if !body.len().is_multiple_of(2) {
        return None;
    }
    let mut out = Vec::new();
    for pair in body.as_bytes().chunks_exact(2) {
        let hi = (char::from(*pair.first()?)).to_digit(16)?;
        let lo = (char::from(*pair.get(1)?)).to_digit(16)?;
        out.push(u8::try_from(hi.checked_shl(4)?.checked_add(lo)?).ok()?);
    }
    Some(out)
}

/// A `0x` prefixed hex number as a `u64`.
pub(super) fn u64_hex(hex: &str) -> Option<u64> {
    let body = hex.strip_prefix("0x")?;
    u64::from_str_radix(body, 16).ok()
}
