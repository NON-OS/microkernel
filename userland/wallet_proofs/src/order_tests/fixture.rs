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

//! A purchase as the market gave it, and the order the meter would make of
//! it, its words built by nonos_nox_chunks' own OrderTerms.

use nonos_nox_chunks::{refund_commit, OrderTerms};
use nonos_nox_config::{address, Contract, DISPUTE_WINDOW};
use nonos_payment_proto::{OfferMade, OrderMade, OrderNew};

use super::deposit_fixture::K;
use crate::order::buy::Buy;

pub const NOW: u64 = 1_800_000_000;
pub const REFUND_TO: [u8; 20] = [0x44; 20];
/// The wallet's own refundSalt for REFUND_TO, as ORDER_NEW carries it.
pub const REFUND_SALT: [u8; 32] = [0x66; 32];
pub const REFUND_PAIR: ([u8; 20], [u8; 32]) = (REFUND_TO, REFUND_SALT);

pub fn buy() -> Buy {
    Buy {
        chain_id: 11_155_111,
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [3; 32],
        payee: [0x55; 20],
        amount: 4_000_000_000_000_000_000,
        chunk_root: [6; 32],
        content_len: 70_000,
        url: b"https://agent.example/escrow".to_vec(),
    }
}

pub fn offer() -> OfferMade {
    OfferMade { offer_id: [7; 32], cipher_root: [8; 32], key_commit: [9; 32] }
}

/// What the meter answers for `n`: its terms as the escrow encodes them,
/// refundCommit over `n`'s own refundTo and refundSalt.
pub fn made(n: &OrderNew) -> OrderMade {
    let terms = OrderTerms {
        listing_id: n.listing_id,
        release_id: n.release_id,
        bond_id: n.bond_id,
        payee: n.payee,
        token: address(n.chain_id, Contract::NoxToken).unwrap(),
        amount: n.amount,
        delivery_deadline: n.deadline,
        dispute_window: DISPUTE_WINDOW,
        chunk_root: n.chunk_root,
        content_len: n.content_len,
        cipher_root: n.cipher_root,
        key_commit: n.key_commit,
        refund_commit: refund_commit(&K, &n.refund_to, &n.refund_salt),
        salt: [0xBB; 32],
    };
    OrderMade { order_id: [0xCC; 32], deposit: [0xDD; 20], terms: terms.words() }
}
