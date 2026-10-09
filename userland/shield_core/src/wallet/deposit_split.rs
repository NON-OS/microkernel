//! One typed amount as several standard deposits: the fewest sizes of 1, 2 or 5 followed by zeros
//! inside the range of the pool, largest first. Each digit of the amount takes at most three, and an
//! amount the sizes cannot make is refused whole, so no remainder is ever left behind.

use crate::net::asset::Asset;

/// The most deposits one review holds.
pub const MAX_PIECES: usize = 20;

/// The deposits that make `units` of `asset`, largest first, or none. The whole may pass what one
/// note holds, and each deposit never does.
pub fn pieces(units: u128, asset: &Asset) -> Option<Vec<u64>> {
    let mut left = units;
    let mut out = Vec::new();
    for size in sizes(asset) {
        while left >= u128::from(size) {
            left = left.checked_sub(u128::from(size))?;
            out.push(size);
            if out.len() > MAX_PIECES {
                return None;
            }
        }
    }
    (left == 0 && !out.is_empty()).then_some(out)
}

/// Every standard size of `asset` inside its range, largest first.
fn sizes(asset: &Asset) -> Vec<u64> {
    let mut out = Vec::new();
    let mut ten: u64 = 1;
    loop {
        for digit in [5u64, 2, 1] {
            if let Some(size) = digit.checked_mul(ten).filter(|s| asset.allows(*s)) {
                out.push(size);
            }
        }
        match ten.checked_mul(10) {
            Some(next) => ten = next,
            None => break,
        }
    }
    out.sort_unstable_by(|a, b| b.cmp(a));
    out
}

#[cfg(test)]
#[path = "deposit_split_test.rs"]
mod deposit_split_test;
