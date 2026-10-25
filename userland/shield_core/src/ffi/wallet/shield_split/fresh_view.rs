//! A SHIELD_FROM_FRESH review as the screen gets it (v1.4.56): the split,
//! what stays at the address, and the ETH it lacks when that is the reason.

use super::view::{refused, ShieldSplit};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct FreshShield {
    pub split: ShieldSplit,
    /// NOX below the smallest standard deposit, left at the address, in wei.
    pub remainder: u128,
    /// The ETH the address lacks for the gas, in wei; 0 unless that is why.
    pub short_eth: u128,
}

pub(super) fn said(why: &str, remainder: u128) -> FreshShield {
    FreshShield { split: refused(why), remainder, short_eth: 0 }
}
