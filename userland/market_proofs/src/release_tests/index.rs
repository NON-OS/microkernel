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

use alloc::string::String;
use alloc::vec;

use nonos_marketplace_abi::{MarketplaceEntry, MarketplaceIndex, PriceKind, PriceModel, TokenInfo};

use crate::readiness_tests::release;

pub(crate) fn index() -> MarketplaceIndex {
    let (mut first, mut second) = (release("x86_64-linux", 0), release("x86_64-linux", 0));
    first.release_id = String::from("jq@1.7.1-r0");
    second.release_id = String::from("jq@1.6-r4");
    let entry = MarketplaceEntry {
        listing_id: String::from("linux.jq"),
        capsule_id: [0; 32],
        name: String::from("jq"),
        publisher_name: String::from("Alpine v3.20"),
        publisher_pubkey: [0; 32],
        publisher_eth_address: [0; 20],
        description: String::new(),
        logo_hash: [0; 32],
        price: PriceModel { kind: PriceKind::Free, amount_atomic: 0, period_seconds: 0 },
        token: TokenInfo {
            symbol: String::new(),
            decimals: 18,
            chain_id: 1,
            contract_address: vec![],
        },
        commerce: Some(crate::abi_v3::fixtures::free_commerce()),
        releases: vec![first, second],
    };
    MarketplaceIndex {
        schema_version: 3,
        operator_id: String::from("nonos.marketplace.v1"),
        operator_pubkey: [0; 32],
        published_at_ms: 0,
        serial: 1,
        entries: vec![entry],
        index_signature: vec![],
    }
}
