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

//! A purchase end to end through its pure parts (B8, the plan): the store
//! names a listing, a release and a chain, and nothing else; the price
//! quoted, bought, ordered and funded is the market block's unit_price.

use nonos_payment_proto::Names;

use super::fixture::{made, offer, NOW, REFUND_SALT};
use super::gate_fixture::{names, FRESH};
use super::market_drive::{escrow_market, escrow_market_for, read_through};
use super::read_fixture::CHAIN;
use crate::order::buy_gate::{buyable, to_quote};
use crate::order::fees_kind::{exact, Fees};
use crate::order::fund_ask::fund_ask;
use crate::order::pending_step::Step;
use crate::order::read::Named;

const PRICE: u128 = 7_250_000_000_000_000_003;

#[test]
fn names_and_a_chain_make_a_buy_at_the_blocks_unit_price() {
    let pairs: [(&[u8], &[u8]); 4] = [
        (b"price=1", b"1.2.3"),
        (b"9999999999999999999", b""),
        (b"app", b"amount=0"),
        (b"x", b"1"),
    ];
    for (listing, release) in pairs {
        let n = Names { listing: listing.to_vec(), release: release.to_vec(), ..names() };
        let read = read_through(&n, &escrow_market_for(&n, PRICE)).1;
        assert!(matches!(&read, Ok(Named::Escrow(b)) if b.amount == PRICE), "{n:?}");
        assert_eq!(to_quote(&Step::Consent, &read, &Fees::Unasked), Some(exact(PRICE)));
        let quoted = Fees::Quoted("0.01".into());
        let b = buyable(&Step::Consent, &read, &quoted).unwrap();
        let order = b.order_new(&offer(), (FRESH, REFUND_SALT), NOW);
        assert_eq!((order.amount, order.refund_to), (PRICE, FRESH), "refundTo: the fresh one");
        let funded = fund_ask(&Step::Fund(Box::new(made(&order))), &read, &quoted).unwrap();
        assert_eq!(funded.amount, exact(PRICE));
    }
}

#[test]
fn a_post_carries_names_only_and_their_chain_must_be_the_markets() {
    let n = names();
    let post = n.post().unwrap();
    assert_eq!(Names::parse(&post[1..]), Some(n.clone()), "TAKE hands on what POST carried");
    let priced = [&post[1..], &PRICE.to_le_bytes()[..]].concat();
    assert_eq!(Names::parse(&priced), None, "a price after the names is no purchase");
    let elsewhere = Names { chain_id: CHAIN + 1, ..n };
    let read = read_through(&elsewhere, &escrow_market(PRICE)).1;
    assert_eq!(read, Err("The market sells this release on another chain."));
}
