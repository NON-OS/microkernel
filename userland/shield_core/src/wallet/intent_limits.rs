//! The numeric limits behind the intent rules.

use crate::notes::MAX_VALUE;

/// The unshield fee cap in basis points of the public amount, the pool's `MAX_FEE_BPS`.
pub const MAX_FEE_BPS: u128 = 50;
const BPS: u128 = 10_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PoolRules {
    pub words_per_intent: u8,
    /// The largest private transfer fee, in units, read from chain. Zero means no fee.
    pub max_relay_fee: u64,
}

impl PoolRules {
    pub fn names_fee_recipient(&self) -> bool {
        self.words_per_intent >= 12
    }
}

/// Every value at most `MAX_VALUE`, and the price a canonical field element.
pub(super) fn in_range(values: &[u64], price: u64) -> bool {
    values.iter().all(|v| *v <= MAX_VALUE) && price < nonos_stark::field::P
}

/// `fee * 10^4 <= public_amount * MAX_FEE_BPS`, as the pool computes it.
pub(super) fn fee_within_cap(fee: u64, public_amount: u64) -> bool {
    let fee_side = u128::from(fee).checked_mul(BPS);
    let cap_side = u128::from(public_amount).checked_mul(MAX_FEE_BPS);
    matches!((fee_side, cap_side), (Some(f), Some(c)) if f <= c)
}

/// Whether the statement can carry `addr`. On an eleven word pool a limb at or above p cannot.
pub(super) fn representable(addr: &[u8; 20], pool: &PoolRules) -> bool {
    if pool.names_fee_recipient() {
        return true;
    }
    let limb = |range: core::ops::Range<usize>| {
        let mut b = [0u8; 8];
        b.copy_from_slice(addr.get(range).unwrap_or(&[0; 8]));
        u64::from_be_bytes(b)
    };
    limb(12..20) < nonos_stark::field::P && limb(4..12) < nonos_stark::field::P
}
