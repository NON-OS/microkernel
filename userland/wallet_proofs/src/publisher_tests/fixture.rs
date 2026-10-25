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

//! One sold entry whose payee is ME and whose bond is ME's for KEY, with an
//! escrow release and, by switching the mode, a package release.

use nonos_marketplace_abi::{bond_id, Commerce, Keccak, MarketplaceEntry};
use nonos_marketplace_abi::{PriceKind, PriceModel, SettleMode, TokenInfo};
use sha3::{Digest, Keccak256};

use super::release::release;

pub struct K;
impl Keccak for K {
    fn keccak256(&self, d: &[u8]) -> [u8; 32] {
        Keccak256::digest(d).into()
    }
}

pub const ME: [u8; 20] = [0x11; 20];
pub const KEY: [u8; 32] = [0x22; 32];

pub fn entry(mode: SettleMode) -> MarketplaceEntry {
    let commerce = Commerce {
        payee: ME,
        bond_id: bond_id(&K, &ME, &KEY),
        unit: [0; 32],
        unit_price: 1_000,
        min_charge: 1,
        settle_mode: mode,
        proof_reqs: vec![],
        binding_sig: [0; 65],
    };
    MarketplaceEntry {
        listing_id: "linux.qwen-small".into(),
        capsule_id: [0; 32],
        name: "qwen".into(),
        publisher_name: "p".into(),
        publisher_pubkey: KEY,
        publisher_eth_address: [0; 20],
        description: String::new(),
        price: PriceModel { kind: PriceKind::OneTime, amount_atomic: 1_000, period_seconds: 0 },
        token: TokenInfo {
            symbol: "NOX".into(),
            decimals: 18,
            chain_id: 1,
            contract_address: vec![],
        },
        commerce: Some(commerce),
        releases: vec![release("qwen-small@1"), release("qwen-small@2")],
    }
}
