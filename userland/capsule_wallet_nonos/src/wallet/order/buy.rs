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
 * An escrow purchase as the person confirmed it (B8, the plan, the plan),
 * and its requests: ORDER_OFFER, then ORDER_NEW on the offer checked.
 */

use alloc::vec::Vec;

use nonos_nox_config::{DELIVER_MARGIN, MAX_DELIVERY, ORDER_MIN_LEAD};
use nonos_payment_proto::{OfferAsk, OfferMade, OrderNew};

/// The seller's time to deliver: two days past the escrow's least lead.
pub const DELIVERY_ASKED: u64 = ORDER_MIN_LEAD + 2 * 24 * 3_600;
const _: () = assert!(DELIVERY_ASKED <= MAX_DELIVERY - DELIVER_MARGIN);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Buy {
    pub chain_id: u64,
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
    pub bond_id: [u8; 32],
    pub payee: [u8; 20],
    pub amount: u128,
    pub chunk_root: [u8; 32],
    pub content_len: u64,
    /// The seller agent's escrow endpoint, https.
    pub url: Vec<u8>,
}

impl Buy {
    pub fn offer_ask(&self) -> OfferAsk {
        OfferAsk {
            chain_id: self.chain_id,
            listing_key: self.listing_id,
            release_key: self.release_id,
            chunk_root: self.chunk_root,
            content_len: self.content_len,
            url: self.url.clone(),
        }
    }

    /// The order on `offer`, refunded to `refund` (refundTo, refundSalt), at `now` (s).
    pub fn order_new(&self, offer: &OfferMade, refund: ([u8; 20], [u8; 32]), now: u64) -> OrderNew {
        OrderNew {
            chain_id: self.chain_id,
            listing_id: self.listing_id,
            release_id: self.release_id,
            bond_id: self.bond_id,
            payee: self.payee,
            amount: self.amount,
            chunk_root: self.chunk_root,
            content_len: self.content_len,
            cipher_root: offer.cipher_root,
            key_commit: offer.key_commit,
            deadline: now + DELIVERY_ASKED,
            refund_to: refund.0,
            refund_salt: refund.1,
        }
    }
}
