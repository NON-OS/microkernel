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

//! A kept offer from a real answer, and an order whose terms commit to it.

use nonos_nox_chunks::OrderTerms;
use nonos_nox_meter::escrow::KeptOffer;
use nonos_nox_meter::flow::Next;
use nonos_nox_meter::orders::Order;

use super::offer_run::{run, work};
use super::seller::{root, sealed};

pub fn kept() -> KeptOffer {
    let w = work(root());
    let Ok(Next::KeepOffer(k, _)) = run(w.clone(), &sealed(&w.buyer_pk, &[1; 32], &[2; 32])) else {
        panic!()
    };
    *k
}

pub fn order(k: &KeptOffer) -> Order {
    let terms = OrderTerms {
        listing_id: k.listing_key,
        release_id: k.release_key,
        bond_id: [3; 32],
        payee: [4; 20],
        token: [5; 20],
        amount: 1_000,
        delivery_deadline: 9,
        dispute_window: 8,
        chunk_root: k.chunk_root,
        content_len: k.content_len,
        cipher_root: k.offer.cipher_root,
        key_commit: k.offer.key_commit,
        refund_commit: [6; 32],
        salt: [7; 32],
    };
    let (order_id, deposit, refund_to, refund_salt) = ([0xd0; 32], [0xde; 20], [8; 20], [9; 32]);
    Order {
        chain_id: k.chain_id,
        order_id,
        deposit,
        terms: terms.words(),
        refund_to,
        refund_salt,
        refund_taken: false,
        plain: false,
    }
}
