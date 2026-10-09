//! The arithmetic of a two in two out transfer.
//!
//! A transfer spends two notes and creates two, the recipient's and the change. What goes
//! in must equal what comes out plus the fee, and that equation is where a wallet can lose
//! money silently, through a sum that wraps or a difference below zero.

//! Note selection lives in the core. The balance lives here so it can carry a proof.

/// The gwei a transfer must cover and the change it returns, given the two
/// notes it spends. Nothing if the amount and fee do not fit in a `u64`, if
/// the two notes together do not, or if they do not cover the amount and fee.
///
/// Every operation is the checked one. A match instead of the question mark
/// operator, so the translation is a case split a reader can follow.
pub fn balance(first: u64, second: u64, amount: u64, fee: u64) -> Option<(u64, u64)> {
    let needed = match amount.checked_add(fee) {
        None => return None,
        Some(needed) => needed,
    };
    let total = match first.checked_add(second) {
        None => return None,
        Some(total) => total,
    };
    match total.checked_sub(needed) {
        None => None,
        Some(change) => Some((needed, change)),
    }
}
