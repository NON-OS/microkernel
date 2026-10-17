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

//! The two requests a purchase sends: ORDER_OFFER from the listing as
//! given, ORDER_NEW on the offer checked, with a deadline the escrow takes.

use nonos_nox_config::{DELIVER_MARGIN, MAX_DELIVERY, ORDER_MIN_LEAD};
use nonos_payment_proto::{OfferAsk, OrderNew};

use super::fixture::{buy, offer, NOW, REFUND_PAIR};
use crate::order::buy::DELIVERY_ASKED;

#[test]
fn the_offer_names_the_listing_release_chunks_and_agent() {
    let a = buy().offer_ask();
    assert_eq!((a.chain_id, a.listing_key, a.release_key), (11_155_111, [1; 32], [2; 32]));
    assert_eq!((a.chunk_root, a.content_len), ([6; 32], 70_000));
    let wire = a.encode().unwrap();
    assert_eq!(OfferAsk::parse(&wire), Some(a), "the meter reads what the wallet sends");
}

#[test]
fn the_order_takes_the_offers_roots_and_a_deadline_in_the_window() {
    let n = buy().order_new(&offer(), REFUND_PAIR, NOW);
    assert_eq!((n.cipher_root, n.key_commit), ([8; 32], [9; 32]));
    assert_eq!((n.bond_id, n.payee, n.amount), ([3; 32], [0x55; 20], buy().amount));
    assert_eq!((n.refund_to, n.refund_salt), REFUND_PAIR, "the wallet's own pair");
    assert!(n.deadline >= NOW + ORDER_MIN_LEAD);
    assert!(n.deadline <= NOW + MAX_DELIVERY - DELIVER_MARGIN);
    assert_eq!(n.deadline, NOW + DELIVERY_ASKED);
    assert_eq!(OrderNew::parse(&n.encode()), Some(n), "the meter reads what the wallet sends");
}
