//! A NOX transfer from the token state read per send: blacklist, pause, a buy fee
//! from a pair, sell fee to one, transfer fee otherwise, none if either is exempt.

/// The token state one transfer depends on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Rules {
    pub paused: bool,
    pub blocked: bool,
    /// Buy, sell and transfer fees, in basis points.
    pub fees: [u16; 3],
    pub pair_from: bool,
    pub pair_to: bool,
    pub exempt: bool,
}

/// What reaches the recipient, or why the token would refuse.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Outcome {
    Arrives { amount: u128, fee: u128, bps: u16 },
    Refused(&'static str),
}

pub(super) fn outcome(rules: &Rules, amount: u128) -> Outcome {
    if rules.paused {
        return Outcome::Refused("NOX transfers are paused on this network.");
    }
    if rules.blocked {
        return Outcome::Refused("The token blocks the sender or the recipient.");
    }
    let [buy, sell, transfer] = rules.fees;
    let bps = match (rules.exempt, rules.pair_from, rules.pair_to) {
        (true, _, _) => 0,
        (false, true, _) => buy,
        (false, false, true) => sell,
        (false, false, false) => transfer,
    };
    // The split is the verified kernel's, proved in Lean.
    match nox_verified::fee::split_fee(amount, bps) {
        Some((arrives, fee)) => Outcome::Arrives { amount: arrives, fee, bps },
        None => Outcome::Refused("The token's fee would take the whole amount."),
    }
}

#[cfg(test)]
#[path = "nox_rules_test.rs"]
mod nox_rules_test;

#[cfg(kani)]
#[path = "nox_rules_kani.rs"]
mod nox_rules_kani;
