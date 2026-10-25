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

//! A named purchase's terms are read one call at a time, in order (B8,
//! v1.4.36); an answer that does not read ends the reading there.

use nonos_market_proto::{listing_body, pair_body, ready_body as ready_ask};
use nonos_market_proto::{OP_GET_APP, OP_GET_COMMERCE, OP_GET_RELEASE, OP_INSTALL_READY};
use nonos_marketplace_abi::SettleMode;

use super::deposit_fixture::K;
use super::fixture::buy;
use super::gate_fixture::names;
use super::ids_fixture::{bought, version};
use super::market_drive::{escrow_market, read_through, Market};
use super::read_fixture::{commerce, escrow, CHAIN};
use crate::order::read::Named;
use crate::order::reading_step::{call, Read, Reading, UNREAD};
use crate::order::terms::answer::answered;

fn market() -> Market {
    escrow_market(buy().amount)
}

#[test]
fn the_market_is_asked_commerce_then_release_then_ready() {
    let n = names();
    let (calls, read) = read_through(&n, &market());
    let pair = pair_body(&n.listing, &n.release).unwrap();
    let ready = ready_ask(&n.listing, &n.release, CHAIN).unwrap();
    let (app, rel) = ((OP_GET_APP, listing_body(&n.listing).unwrap()), OP_GET_RELEASE);
    let want = [(OP_GET_COMMERCE, pair.clone()), (rel, pair), (OP_INSTALL_READY, ready), app];
    assert_eq!(calls, want);
    assert_eq!(read, Ok(Named::Escrow(bought())));
}

#[test]
fn an_answer_that_does_not_read_ends_the_reading_at_its_call() {
    for (at, short) in (0..3).flat_map(|at| [(at, true), (at, false)]) {
        let mut m = market();
        m[at] = if short { m[at][..m[at].len() - 1].to_vec() } else { Vec::new() };
        let (calls, read) = read_through(&names(), &m);
        assert_eq!((calls.len(), read), (at + 1, Err(UNREAD)), "answer {at}");
    }
}

#[test]
fn each_answer_in_hand_names_the_next_call() {
    let (n, m) = (names(), market());
    assert_eq!(call(&n, &Reading::Commerce).unwrap().0, OP_GET_COMMERCE);
    let Ok(Read::More(at)) = answered(&K, &n, Reading::Commerce, &m[0]) else { panic!() };
    assert!(matches!(&at, Reading::Release(c) if c.block == commerce(SettleMode::Escrow).block));
    assert_eq!(call(&n, &at).unwrap().0, OP_GET_RELEASE);
    let Ok(Read::More(at)) = answered(&K, &n, at, &m[1]) else { panic!() };
    assert!(matches!(&at, Reading::Ready(_, e, v) if **e == escrow() && *v == version(&n)));
    assert_eq!(call(&n, &at).unwrap().0, OP_INSTALL_READY);
    let Ok(Read::More(at)) = answered(&K, &n, at, &m[2]) else { panic!() };
    assert!(matches!(&at, Reading::App(_, b, _) if **b == Named::Escrow(bought())));
    assert_eq!(call(&n, &at).unwrap().0, OP_GET_APP);
    let Ok(Read::Done(read)) = answered(&K, &n, at, &m[3]) else { panic!() };
    assert_eq!(read.map(|t| t.named), Ok(Named::Escrow(bought())));
}
