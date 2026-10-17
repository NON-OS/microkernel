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

//! The meter's ORDER_NEW reply with the id and deposit nonos_nox_chunks
//! derives, on an escrow and forwarder hash made up here: the config pins
//! neither yet (Deployment.escrow and forwarder_initcode_hash are None).

use nonos_marketplace_abi::Keccak;
use nonos_nox_chunks::{deposit_address, order_id, OrderTerms};
use nonos_payment_proto::{OrderMade, OrderNew};
use sha3::{Digest, Keccak256};

use super::fixture::made;
use crate::order::deposit_check::Pinned;
use crate::order::pending_step::Step;
use crate::order::purchase::{after_order, After};

pub struct K;
impl Keccak for K {
    fn keccak256(&self, d: &[u8]) -> [u8; 32] {
        Keccak256::digest(d).into()
    }
}

/// Made up, never a deployed address or hash.
pub const AT: Pinned = Pinned { escrow: [0xE5; 20], forwarder: [0xF7; 32] };

/// The id and deposit the shared derivation makes of `words` on `at`.
pub fn ids(chain_id: u64, words: &[[u8; 32]; 14], at: &Pinned) -> ([u8; 32], [u8; 20]) {
    let t = OrderTerms::from_words(words).unwrap();
    let id = order_id(&K, chain_id, &at.escrow, &t);
    (id, deposit_address(&K, &at.escrow, &id, &at.forwarder))
}

/// What the meter answers for `n` on AT, its id and deposit its own.
pub fn made_at(n: &OrderNew) -> OrderMade {
    let mut m = made(n);
    (m.order_id, m.deposit) = ids(n.chain_id, &m.terms, &AT);
    m
}

/// made_at with the meter's salt byte `b`, and refundCommit `commit` if Some.
pub fn salted(n: &OrderNew, b: u8, commit: Option<[u8; 32]>) -> OrderMade {
    let mut m = made(n);
    (m.terms[12], m.terms[13][0]) = (commit.unwrap_or(m.terms[12]), b);
    (m.order_id, m.deposit) = ids(n.chain_id, &m.terms, &AT);
    m
}

/// ORDER_NEW's job result for `n` on AT.
pub fn ordered(n: &OrderNew, r: Result<&[u8], i32>) -> After {
    after_order(&K, n, Some(AT), r)
}

/// The step poll moves to on `a` (order/step.rs).
pub fn step_of(a: After) -> Step {
    match a {
        After::Fund(m) => Step::Fund(m),
        After::Failed(why) => Step::Failed(why),
        other => panic!("not an ORDER_NEW end: {other:?}"),
    }
}
