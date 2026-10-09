//! The money arithmetic of a public send: the token fee, the fee offered, the nonce, the cover.
//! Written for Aeneas with a case split per checked operation. The Lean extraction exists and
//! its proofs are not written yet. A send offers at most 1,000 gwei a gas, and a 5 gwei tip.

pub const BPS: u128 = 10_000;
pub const CEILING: u128 = 1_000_000_000_000;
pub const MAX_TIP: u128 = 5_000_000_000;

/// What arrives and the fee taken at `bps`, or nothing if the fee takes all or overflows.
pub fn split_fee(amount: u128, bps: u16) -> Option<(u128, u128)> {
    match amount.checked_mul(bps as u128) {
        None => None,
        Some(product) => {
            let fee = product / BPS;
            if fee < amount {
                // Cannot fail after the comparison, but the crate allows no bare arithmetic.
                match amount.checked_sub(fee) {
                    None => None,
                    Some(arrives) => Some((arrives, fee)),
                }
            } else {
                None
            }
        }
    }
}

/// The tip, held to `MAX_TIP`, and the cap, twice the base fee plus the tip, up to `CEILING`.
pub fn offer(base: u128, suggested: u128) -> Option<(u128, u128)> {
    let tip = if suggested < MAX_TIP { suggested } else { MAX_TIP };
    match base.checked_mul(2) {
        None => None,
        Some(doubled) => match doubled.checked_add(tip) {
            None => None,
            Some(cap) => {
                if cap <= CEILING {
                    Some((tip, cap))
                } else {
                    None
                }
            }
        },
    }
}

/// The larger of the pending count and the recorded nonce, so a send never reuses a nonce.
pub fn next_nonce(pending: u64, recorded: u64) -> u64 {
    if pending < recorded {
        recorded
    } else {
        pending
    }
}

/// Whether `balance` covers `amount` plus `fee`, with no sum wrapping small enough to pass.
pub fn covers(balance: u128, amount: u128, fee: u128) -> bool {
    match amount.checked_add(fee) {
        None => false,
        Some(need) => need <= balance,
    }
}
