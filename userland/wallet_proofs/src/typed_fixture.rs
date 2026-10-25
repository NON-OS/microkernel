// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! Deployments the config does not have yet, for the typed-data proofs:
//! the config's own names no marketplace contract, so without these no
//! address could be shown reaching a domain.

use nonos_nox_config::{Chain, Deployed, Deployment};

const fn at(n: u8) -> Option<Deployed> {
    Some(Deployed { address: [n; 20], block: 0 })
}

/// Every contract at an address of its own, so one read from the wrong
/// field shows as the wrong address. Settlement's is 0x07 x 20.
pub static EVERY: Deployment = Deployment {
    nox_token: at(1),
    settlement: at(7),
    escrow: at(3),
    publisher_bond: at(4),
    listing_registry: at(5),
    proof_anchor: at(6),
    shield_pool: at(8),
    amount_policy: at(9),
};

/// EVERY without the two contracts the keyring signs under.
static NEITHER: Deployment = Deployment { settlement: None, publisher_bond: None, ..EVERY };

/// Neither of those on mainnet, EVERY on Sepolia.
pub fn sepolia_only(chain: Chain) -> &'static Deployment {
    match chain {
        Chain::Mainnet => &NEITHER,
        Chain::Sepolia => &EVERY,
    }
}
