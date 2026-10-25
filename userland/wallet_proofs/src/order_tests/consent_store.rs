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

//! The consent is the wallet's, said in words, and what it buys is named
//! by the market, whatever the store called it: a POST whose names suggest
//! another thing shows the market's title, the store's text only on the
//! detail line as what the store called it.

use nonos_payment_proto::Names;

use super::consent_terms::WALLET;
use super::deposit_fixture::K;
use super::gate_fixture::names;
use super::record_fixture::{read, recorded, PAYEE_TEXT};
use crate::order::fees_kind::Fees;
use crate::order::terms::lines::lines;

#[test]
fn the_wallet_says_by_its_own_name_that_it_asks_and_whom_it_pays() {
    let n = names();
    let (b, r) = read(&n, &recorded(&n, b"Quiet Notes"));
    let l = lines(&K, WALLET, &n, &r, &b, &Fees::Quoted("0.0125".into()));
    assert!(l.lead.starts_with("NONOS Wallet is asking you, not the store: pay "));
    assert!(l.lead.contains(PAYEE_TEXT), "the payee in full, in words");
    let other = lines(&K, "Another Name", &n, &r, &b, &Fees::Quoted("0.0125".into()));
    assert!(other.lead.starts_with("Another Name is asking"), "the name is the wallet's own");
}

#[test]
fn a_post_whose_names_differ_from_the_market_record_shows_the_markets() {
    let store = Names {
        listing: b"linux.free-gift-card".to_vec(),
        release: b"linux.free-gift-card@9.9.9".to_vec(),
        ..names()
    };
    let post = store.post().unwrap();
    let taken = Names::parse(&post[1..]).unwrap();
    let (b, r) = read(&taken, &recorded(&taken, b"Paid Backup Service"));
    let l = lines(&K, WALLET, &taken, &r, &b, &Fees::Quoted("0.0125".into()));
    let release = "linux.free-gift-card@9.9.9";
    assert_eq!(l.rows[0], ("buying", format!("Paid Backup Service, release {release}")));
    let others = l.rows[1..].iter().map(|r| &*r.1);
    for line in [l.lead.as_str(), &l.escrow].into_iter().chain(others) {
        assert!(!line.contains("free-gift"), "never the store's names: {line}");
    }
    assert!(l.detail.ends_with(&format!("the store called it linux.free-gift-card, {release}.")));
}
