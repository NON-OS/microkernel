//! The arithmetic under every amount a screen shows or reads.
//!
//! A note holds a count of base units in a `u64`. NOX has eighteen decimals, so one token is
//! 10^18 base units, and the largest note, 2^64 - 1 units, is 18.446744073709551615 tokens.

//! Converting is a division and a multiplication, where money can silently go wrong: a
//! fraction carrying into the whole, or a product that wraps.
//!
//! String handling around these lives in the core. The arithmetic lives here so it can
//! carry a proof.

/// Base units in one whole token, eighteen decimals. Written out, not raised at runtime, so
/// producing it cannot overflow.
pub const SCALE: u64 = 1_000_000_000_000_000_000;

/// A count of base units as the two numbers a screen prints: the whole tokens
/// and the base units left over, which is always below [`SCALE`].
pub fn split(units: u64) -> (u64, u64) {
    (units / SCALE, units % SCALE)
}

/// The base units a whole part and a fraction describe, or nothing if the fraction is not
/// below [`SCALE`] or the total would not fit.
///
/// Refusing to wrap is the purpose here. A wrapped product is a balance smaller than what
/// was typed, and a person does not notice that kind of wrong until later, so an overflow
/// returns nothing.
pub fn combine(whole: u64, fraction: u64) -> Option<u64> {
    if fraction >= SCALE {
        return None;
    }
    // A match, not the question mark operator. Both compile the same, but the match
    // translates to a case split and the operator to a trait call, and a proof over a case
    // split is one a reader can follow.

    match whole.checked_mul(SCALE) {
        None => None,
        Some(scaled) => scaled.checked_add(fraction),
    }
}
