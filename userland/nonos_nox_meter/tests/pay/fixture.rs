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

//! A request body and the release it should match.

use nonos_marketplace_abi::{commerce_bytes, Commerce, SettleMode};
use nonos_nox_meter::pay::Offer;

pub const NOX: [u8; 20] = [0x0a; 20];

pub fn commerce() -> Commerce {
    Commerce {
        payee: [0xbb; 20],
        bond_id: [9; 32],
        unit: [3; 32],
        unit_price: 7,
        min_charge: 30,
        settle_mode: SettleMode::Voucher,
        proof_reqs: vec![],
        binding_sig: [0; 65],
    }
}

pub fn offer(c: &Commerce) -> Offer {
    let (listing_id, release_hash, chain_id) = ([1; 32], [2; 32], 1);
    Offer {
        trusted: true,
        listing_id,
        release_hash,
        chain_id,
        token: NOX.to_vec(),
        block: commerce_bytes(c),
    }
}

pub fn body(listing: &[u8], release: &[u8]) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&[1; 32]);
    b.extend_from_slice(&[2; 32]);
    b.extend_from_slice(&[0xbb; 20]);
    b.extend_from_slice(&[3; 32]);
    b.extend_from_slice(&5u64.to_le_bytes());
    b.extend_from_slice(&7u128.to_le_bytes());
    b.extend_from_slice(&[4; 32]);
    b.extend_from_slice(&1u64.to_le_bytes());
    for s in [listing, release] {
        b.extend_from_slice(&(s.len() as u32).to_le_bytes());
        b.extend_from_slice(s);
    }
    b
}
