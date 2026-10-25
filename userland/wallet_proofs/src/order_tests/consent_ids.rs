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

//! The market's answer must be for the names asked (B8): GET_APP echoes
//! the listing's text, GET_RELEASE the release's, and GET_COMMERCE's ids
//! are their hashes under the publisher's key. Any other answer is a
//! refusal, and no Buy.

use nonos_marketplace_abi::SettleMode;
use nonos_payment_proto::Names;

use super::gate_fixture::names;
use super::ids_fixture::{commerce_for, KEY, TITLE};
use super::market_answers::{app_body, commerce_body, release_body};
use super::market_drive::{escrow_market, escrow_market_for, read_terms, Market};
use super::read_fixture::escrow;
use crate::order::buy_gate::buyable;
use crate::order::fees_kind::Fees;
use crate::order::pending_step::Step;
use crate::order::terms::asked::{OTHER_LISTING, OTHER_RELEASE};

fn no_buy(n: &Names, m: &Market, why: &str) {
    let read = read_terms(n, m).1;
    assert_eq!(read.as_ref().err(), Some(&why), "{why}");
    let read = read.map(|t| t.named);
    assert!(buyable(&Step::Consent, &read, &Fees::Quoted("0.01".into())).is_none());
}

#[test]
fn a_market_answer_for_other_ids_than_the_names_gives_no_buy() {
    let n = names();
    let other =
        Names { listing: b"linux.other".to_vec(), release: b"linux.other@1".to_vec(), ..n.clone() };
    let mut m = escrow_market(1);
    m[0] = escrow_market_for(&other, 1)[0].clone();
    no_buy(&n, &m, OTHER_LISTING);
    let mut m = escrow_market(1);
    m[3] = app_body(b"linux.other", TITLE, &KEY);
    no_buy(&n, &m, OTHER_LISTING);
    let mut m = escrow_market(1);
    m[3] = app_body(&n.listing, TITLE, &[0x2C; 32]);
    no_buy(&n, &m, OTHER_LISTING);
    let mut m = escrow_market(1);
    m[1] = release_body(&escrow(), b"linux.other@1");
    no_buy(&n, &m, OTHER_RELEASE);
    let same_listing = Names { release: b"another release".to_vec(), ..n.clone() };
    let c = commerce_for(&same_listing, SettleMode::Escrow);
    let mut m = escrow_market(1);
    m[0] = commerce_body(&c);
    no_buy(&n, &m, OTHER_RELEASE);
}

#[test]
fn the_default_release_is_the_one_the_market_resolved_and_hashed() {
    let n = Names { release: Vec::new(), ..names() };
    let t = read_terms(&n, &escrow_market_for(&n, 1)).1.unwrap();
    assert_eq!(t.record.release, super::ids_fixture::DEFAULT);
    let mut m = escrow_market_for(&n, 1);
    m[1] = release_body(&escrow(), b"linux.quiet-notes@0.1.0");
    no_buy(&n, &m, OTHER_RELEASE);
}
