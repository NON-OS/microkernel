//! The pure half of the recheck at confirm: the ETH there now against the deposits' values and
//! the most their gas can cost by the review's limits and fees or by today's, whichever is more.

use super::tx::Eip1559;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Recheck {
    /// The held transactions go out as reviewed.
    Fits,
    /// The ETH there no longer pays the values and the most their gas can cost now.
    ShortOfEth,
    /// The ETH would pay, but a deposit now needs more gas, or a higher fee, than it was signed
    /// for, or the chain did not answer in full. It is reviewed again; nothing is sent.
    Moved,
}

/// `ether` held now, `max_fee` the ceiling a fee would take now, and `limits` each deposit's gas
/// limit now, against the reviewed `txs`.
pub fn fit(ether: u128, max_fee: u128, limits: &[u64], txs: &[Eip1559]) -> Recheck {
    if limits.len() != txs.len() {
        return Recheck::Moved;
    }
    let mut need = 0u128;
    let mut moved = false;
    for (tx, &now) in txs.iter().zip(limits) {
        moved |= now > tx.gas || max_fee > tx.max_fee;
        let gas = u128::from(now.max(tx.gas)).checked_mul(max_fee.max(tx.max_fee));
        let Some(next) =
            gas.and_then(|g| g.checked_add(tx.value)).and_then(|c| need.checked_add(c))
        else {
            return Recheck::ShortOfEth;
        };
        need = next;
    }
    match (ether < need, moved) {
        (true, _) => Recheck::ShortOfEth,
        (false, true) => Recheck::Moved,
        (false, false) => Recheck::Fits,
    }
}

#[cfg(test)]
#[path = "shield_recheck_test.rs"]
mod shield_recheck_test;
