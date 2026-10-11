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

//! The gates schema 3 adds: whether the publisher and the payee
//! stand behind what is sold, whether the payee attested to these bytes, and
//! whether a Linux release carries the policy it was sold under. A free or
//! schema 2 listing sells nothing, so the first two do not apply to it.

use nonos_marketplace_abi::{attest_verdict, binding_verdict, commerce_verdict, policy_verdict};
use nonos_marketplace_abi::{
    AttestVerdict, BindingVerdict, Blake3, CapsuleRelease, CommerceVerdict,
};
use nonos_marketplace_abi::{Keccak, MarketplaceEntry, Recover, SettleMode};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MarketGates {
    pub commerce: bool,
    pub attest: bool,
    pub policy: bool,
}

impl MarketGates {
    pub fn all(&self) -> bool {
        self.commerce && self.attest && self.policy
    }
}

pub fn market_gates<H: Keccak + Blake3 + Recover>(
    h: &H,
    entry: &MarketplaceEntry,
    rel: &CapsuleRelease,
    publisher_signed: bool,
) -> MarketGates {
    let sells = entry.commerce.as_ref().is_some_and(|c| c.settle_mode != SettleMode::Free);
    let commerce = !sells
        || (publisher_signed
            && commerce_verdict(h, entry, rel) == CommerceVerdict::Matches
            && binding_verdict(h, h, entry) == BindingVerdict::Valid);
    let attest = !sells || attest_verdict(h, h, entry, rel) == AttestVerdict::Valid;
    let policy = policy_verdict(h, entry, rel).passes();
    MarketGates { commerce, attest, policy }
}

/// Whether the entry sells by escrow, so its pin is the chunk tree.
pub fn escrow(entry: &MarketplaceEntry) -> bool {
    entry.commerce.as_ref().is_some_and(|c| c.settle_mode == SettleMode::Escrow)
}
