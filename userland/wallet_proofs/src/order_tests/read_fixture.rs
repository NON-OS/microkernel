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

//! The market's answers for a named purchase: GET_COMMERCE with its
//! block, the release's escrow fields, and INSTALL_READY.

use nonos_market_proto::{BondCode, Commerce, Readiness, ReleaseEscrow};
use nonos_marketplace_abi::{commerce_bytes, Commerce as Block, SettleMode};

use super::fixture::buy;

pub const CHAIN: u64 = 11_155_111;

fn block(mode: SettleMode) -> Block {
    let b = buy();
    Block {
        payee: b.payee,
        bond_id: b.bond_id,
        unit: nonos_nox_config::UNIT_INSTALL,
        unit_price: b.amount,
        min_charge: 0,
        settle_mode: mode,
        proof_reqs: Vec::new(),
        binding_sig: [0; 65],
    }
}

pub fn commerce(mode: SettleMode) -> Commerce {
    let (listing_id, release_id) = (buy().listing_id, buy().release_id);
    let block = commerce_bytes(&block(mode));
    let (commerce_verdict, binding_verdict) = (0, 0);
    let (release_hash, token_contract) = ([0; 32], Vec::new());
    Commerce {
        commerce_verdict,
        binding_verdict,
        listing_id,
        release_id,
        release_hash,
        chain_id: CHAIN,
        token_contract,
        block,
    }
}

pub fn escrow() -> ReleaseEscrow {
    let b = buy();
    ReleaseEscrow { package_url: b.url, chunk_root: b.chunk_root, content_len: b.content_len }
}

pub fn ready(failing: &[usize], bond: BondCode) -> Readiness {
    let mut gates = [true; 11];
    failing.iter().for_each(|i| gates[*i] = false);
    Readiness { install_ready: failing.is_empty(), gates, bond }
}
