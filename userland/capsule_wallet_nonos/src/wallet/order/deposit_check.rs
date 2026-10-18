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

/*
 * ORDER_NEW's reply recomputed before any Shield withdraw:
 * refundCommit over this wallet's own refundTo and refundSalt, orderId over
 * the terms it asked for, and its CREATE2 deposit, by nonos_nox_chunks as
 * the meter derives them. The salt word alone is the meter's.
 */

use nonos_marketplace_abi::Keccak;
use nonos_nox_chunks::{deposit_address, order_id, refund_commit, OrderTerms};
use nonos_nox_config::{address, Contract, DISPUTE_WINDOW};
use nonos_payment_proto::{OrderMade, OrderNew};

#[path = "deposit_pinned.rs"]
mod pins;
pub use pins::{pinned, Pinned};

pub const UNPINNED: &str = "This chain's escrow is not pinned in the wallet yet; do not fund it.";
pub const REFUND_WRONG: &str =
    "The meter's refund commitment is not to this wallet's address; do not fund it.";
pub const ID_WRONG: &str = "The meter's order id is not the one these terms make; do not fund it.";
pub const DEPOSIT_WRONG: &str = "The meter's deposit address is not this order's; do not fund it.";

/// Whether `m` is the order `sent` makes on `at`'s escrow: its refundCommit,
/// its id, then its deposit address. Any refusal is a sentence the screen shows.
pub fn deposit_check(
    k: &impl Keccak,
    sent: &OrderNew,
    m: &OrderMade,
    at: Option<Pinned>,
) -> Result<(), &'static str> {
    let at = at.ok_or(UNPINNED)?;
    let token = address(sent.chain_id, Contract::NoxToken).map_err(|_| UNPINNED)?;
    let refund = refund_commit(k, &sent.refund_to, &sent.refund_salt);
    (refund == m.terms[12]).then_some(()).ok_or(REFUND_WRONG)?;
    let terms = OrderTerms {
        listing_id: sent.listing_id,
        release_id: sent.release_id,
        bond_id: sent.bond_id,
        payee: sent.payee,
        token,
        amount: sent.amount,
        delivery_deadline: sent.deadline,
        dispute_window: DISPUTE_WINDOW,
        chunk_root: sent.chunk_root,
        content_len: sent.content_len,
        cipher_root: sent.cipher_root,
        key_commit: sent.key_commit,
        refund_commit: refund,
        salt: m.terms[13],
    };
    let id = order_id(k, sent.chain_id, &at.escrow, &terms);
    (id == m.order_id).then_some(()).ok_or(ID_WRONG)?;
    let deposit = deposit_address(k, &at.escrow, &id, &at.forwarder);
    (deposit == m.deposit).then_some(()).ok_or(DEPOSIT_WRONG)
}
