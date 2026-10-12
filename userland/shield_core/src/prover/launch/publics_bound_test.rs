//! An address limb one past its width is refused, never written as a false word.

// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]

use super::super::{pool_words, MASK48};
use super::WITHDRAW_A;
use crate::error::ProveError;

#[test]
fn an_address_limb_one_past_its_width_is_refused() {
    // The recipient's limbs sit at 28..32, the submitter's at 32..36: 48, 48, 48 and 16 bits.
    let mask48 = u64::try_from(MASK48).unwrap_or_default();
    let widths =
        [(28, mask48), (29, mask48), (30, mask48), (31, 0xFFFF), (34, mask48), (35, 0xFFFF)];
    for (at, widest) in widths {
        let mut limbs = WITHDRAW_A;
        limbs[at] = widest;
        assert!(pool_words(&limbs).is_ok(), "limb {at} at its widest");
        limbs[at] = widest + 1;
        assert_eq!(pool_words(&limbs), Err(ProveError::PublicLimb), "limb {at} one past");
    }
    let mut limbs = WITHDRAW_A;
    limbs[31] = u64::MAX;
    assert_eq!(pool_words(&limbs), Err(ProveError::PublicLimb), "a top limb past 16 bits");
}
