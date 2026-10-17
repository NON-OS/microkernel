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

//! ORDER_NEW's plan: the buyer's terms
//! (nonos_nox_chunks::buyer_terms: token the config's NOX, window
//! DISPUTE_WINDOW, the wallet's refundTo behind refundCommit under the
//! wallet's own refundSalt, a deadline in [now + ORDER_MIN_LEAD, now +
//! MAX_DELIVERY - DELIVER_MARGIN]), orderId, and the deposit address the
//! meter computes itself. The capsule then has READ 21 agree, refuses while
//! the policy is paused, and seals the order first.

use nonos_marketplace_abi::Keccak;
use nonos_nox_chunks::{buyer_terms, deposit_address, order_id, Buyer, OrderWrong, Sale};
use nonos_payment_proto::OrderNew;

use super::record::Order;
use crate::error::MeterError;

/// `at` is the config's NOX and escrow on the order's chain, and the
/// forwarder's initcode hash there; `salt` is the order's, fresh. refundSalt
/// is the request's, never the meter's: a zero one is refused.
pub fn order_plan(
    k: &impl Keccak,
    o: &OrderNew,
    at: (&[u8; 20], &[u8; 20], &[u8; 32]),
    salt: [u8; 32],
    now: u64,
) -> Result<Order, MeterError> {
    let (nox, escrow, forwarder) = at;
    (o.refund_salt != [0; 32]).then_some(()).ok_or(MeterError::Mismatch)?;
    let sale = Sale {
        listing_id: o.listing_id,
        release_id: o.release_id,
        bond_id: o.bond_id,
        payee: o.payee,
        amount: o.amount,
        chunk_root: o.chunk_root,
        content_len: o.content_len,
        cipher_root: o.cipher_root,
        key_commit: o.key_commit,
    };
    let (refund_to, refund_salt) = (o.refund_to, o.refund_salt);
    let buyer = Buyer { deadline: o.deadline, refund_to, refund_salt, salt };
    let terms = buyer_terms(k, &sale, nox, &buyer, now).map_err(|w| match w {
        OrderWrong::Empty => MeterError::Mismatch,
        OrderWrong::Deadline => MeterError::Expired,
    })?;
    let id = order_id(k, o.chain_id, escrow, &terms);
    let deposit = deposit_address(k, escrow, &id, forwarder);
    Ok(Order {
        chain_id: o.chain_id,
        order_id: id,
        deposit,
        terms: terms.words(),
        refund_to,
        refund_salt,
        refund_taken: false,
        plain: false,
    })
}
