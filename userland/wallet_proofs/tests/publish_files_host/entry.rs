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

//! A free listing with no releases, the base every document here grows from.

use nonos_marketplace_abi::TokenInfo;
use nonos_marketplace_abi::{Commerce, MarketplaceEntry, PriceKind, PriceModel, SettleMode};

pub fn entry() -> MarketplaceEntry {
    let commerce = Commerce {
        payee: [0x11; 20],
        bond_id: [0x22; 32],
        unit: [0; 32],
        unit_price: 1_000,
        min_charge: 1,
        settle_mode: SettleMode::Free,
        proof_reqs: vec![],
        binding_sig: [0; 65],
    };
    let price = PriceModel { kind: PriceKind::OneTime, amount_atomic: 1_000, period_seconds: 0 };
    let token =
        TokenInfo { symbol: "NOX".into(), decimals: 18, chain_id: 1, contract_address: vec![] };
    MarketplaceEntry {
        listing_id: "linux.qwen-small".into(),
        capsule_id: [0; 32],
        name: "qwen".into(),
        publisher_name: "p".into(),
        publisher_pubkey: [0x22; 32],
        publisher_eth_address: [0; 20],
        description: String::new(),
        logo_hash: [0; 32],
        price,
        token,
        commerce: Some(commerce),
        releases: vec![],
    }
}
