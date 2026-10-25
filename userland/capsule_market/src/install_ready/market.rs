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
//! whether a Linux release carries the policy it was sold under, and whether
//! the price the store shows is the one the meter charges. The sale gates
//! apply to schema 3 releases only: a schema 2 release keeps today's checks
//! until the owner re-seals it (O5), and a free listing sells nothing to
//! attest. The policy gate holds whatever the schema: a linux.* release
//! fetched from an index never installs without its box, and only a shipped
//! tier (a linux.qwen-* listing running "store:/bin/qwenchat") is boxed by the
//! personality's own table; another "store:" url needs its box like any package.

use nonos_marketplace_abi::SettleMode;
use nonos_marketplace_abi::{attest_verdict, binding_verdict, commerce_verdict, policy_verdict};
use nonos_marketplace_abi::{price_verdict, Keccak, MarketplaceEntry, PriceVerdict, Recover};
use nonos_marketplace_abi::{
    AttestVerdict, BindingVerdict, Blake3, CapsuleRelease, CommerceVerdict,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MarketGates {
    pub commerce: bool,
    pub attest: bool,
    pub policy: bool,
    /// The shown price agrees with the commerce block (B6).
    pub price: bool,
}

impl MarketGates {
    pub fn all(&self) -> bool {
        self.commerce && self.attest && self.policy && self.price
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
    let price = price_verdict(entry) != PriceVerdict::Contradicts;
    MarketGates { commerce, attest, policy, price }
}

/// Whether the entry sells by escrow, so its pin is the chunk tree.
pub fn escrow(entry: &MarketplaceEntry) -> bool {
    entry.commerce.as_ref().is_some_and(|c| c.settle_mode == SettleMode::Escrow)
}
