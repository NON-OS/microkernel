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

//! The buyer's terms: what the sale fixes, the token
//! the config's NOX, refundTo committed behind refundCommit, and a deadline
//! in [now + ORDER_MIN_LEAD, now + MAX_DELIVERY - DELIVER_MARGIN], so the
//! seller's agent (reveal_ok) can still deliver inside its margin.

use nonos_marketplace_abi::Keccak;
use nonos_nox_config::{DELIVER_MARGIN, DISPUTE_WINDOW, MAX_DELIVERY, ORDER_MIN_LEAD};

use crate::buyer_types::{Buyer, OrderWrong, Sale};
use crate::order::refund_commit;
use crate::terms::OrderTerms;

pub fn buyer_terms(
    k: &impl Keccak,
    s: &Sale,
    nox: &[u8; 20],
    b: &Buyer,
    now: u64,
) -> Result<OrderTerms, OrderWrong> {
    if s.amount == 0 || s.content_len == 0 || b.refund_to == [0; 20] {
        return Err(OrderWrong::Empty);
    }
    let latest = now.saturating_add(MAX_DELIVERY).saturating_sub(DELIVER_MARGIN);
    if b.deadline < now.saturating_add(ORDER_MIN_LEAD) || b.deadline > latest {
        return Err(OrderWrong::Deadline);
    }
    Ok(OrderTerms {
        listing_id: s.listing_id,
        release_id: s.release_id,
        bond_id: s.bond_id,
        payee: s.payee,
        token: *nox,
        amount: s.amount,
        delivery_deadline: b.deadline,
        dispute_window: DISPUTE_WINDOW,
        chunk_root: s.chunk_root,
        content_len: s.content_len,
        cipher_root: s.cipher_root,
        key_commit: s.key_commit,
        refund_commit: refund_commit(k, &b.refund_to, &b.refund_salt),
        salt: b.salt,
    })
}
