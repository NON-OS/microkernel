//! The value split the note commitment is built from.
//!
//! A note's 64 bit value is hashed as two 32 bit limbs, low half first. The wallet and the
//! settlement contract must agree on that split to the bit: the wallet recomputes a
//! commitment to accept a served record as its note, and the contract to admit a leaf.

//! A disagreement shows up as every record being rejected, on a deployed pool, weeks
//! later.
//!
//! The split is four lines two codebases had to compare by hand, so it carries a proof
//! instead of a conversation.

/// The low half of a value: the bottom 32 bits, widened.
pub fn low(value: u64) -> u64 {
    value & 0xFFFF_FFFF
}

/// The high half: the top 32 bits, shifted down.
pub fn high(value: u64) -> u64 {
    value >> 32
}

/// The value the two halves came from.
///
/// It takes the halves as stored and masks before shifting, so a high half with rubbish
/// above its 32 bits yields the value the commitment would have hashed, not a wider
/// number.
pub fn join(low: u64, high: u64) -> u64 {
    ((high & 0xFFFF_FFFF) << 32) | (low & 0xFFFF_FFFF)
}
