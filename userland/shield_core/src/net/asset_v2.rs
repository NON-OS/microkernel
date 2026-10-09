//! The v2 pool's assets: one flat relay fee, standard amounts inside a range, and the rule that
//! holds a deposit, a send or a withdrawal to them.

use super::asset::{Asset, ETH, NOX};

/// ETH on the v2 pool: from 0.01 ETH, and a flat fee of 0.00005 ETH, 0.5% of the smallest. A note
/// counts wei and holds at most p - 2, about 18.44 ETH, so that is the largest here, not 50.
pub const V2_ETH: Asset = Asset {
    relay_fee: 50_000_000_000_000,
    least: 10_000_000_000_000_000,
    most: 18_446_744_069_414_584_319,
    max_relay_fee: 50_000_000_000_000,
    ..ETH
};

/// NOX on the v2 pool: 1,000 to 5,000,000 NOX, and a flat fee of 5 NOX, 0.5% of the smallest.
pub const V2_NOX: Asset = Asset {
    relay_fee: 5_000_000_000,
    least: 1_000_000_000_000,
    most: 5_000_000_000_000_000,
    max_relay_fee: 5_000_000_000,
    ..NOX
};

/// ETH on the production pool: the v2 range, and a fee read from the policy. The fee shown before
/// a send is a private transfer's at the lowest rung, 0.0005 ETH and 0.0025 ETH. The cap holds a
/// withdrawal of the largest note at 0.50% and the top rung.
pub const PROD_ETH: Asset =
    Asset { relay_fee: 3_000_000_000_000_000, max_relay_fee: 120_000_000_000_000_000, ..V2_ETH };

/// NOX on the production pool: the v2 range, 400 NOX and 2,000 NOX at the lowest rung, and a cap
/// that holds a withdrawal of 5,000,000 NOX at 0.50% and the top rung.
pub const PROD_NOX: Asset =
    Asset { relay_fee: 2_400_000_000_000, max_relay_fee: 50_000_000_000_000, ..V2_NOX };

impl Asset {
    /// Whether `units` may be deposited, sent or withdrawn: a standard size, 1, 2 or 5 followed by
    /// zeros, inside the pool's range. Change stays inside the pool and is not held to this.
    pub fn allows(&self, units: u64) -> bool {
        let unit = nox_prover::policy::unit_for(self.id).unwrap_or(0);
        nox_prover::policy::is_standard(units, unit) && (self.least..=self.most).contains(&units)
    }
}
