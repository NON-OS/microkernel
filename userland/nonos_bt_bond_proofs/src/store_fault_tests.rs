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

//! A keyring that refuses part way: the old bond is still the one kept,
//! and a full book stores nothing.

use crate::faulty::Faulty;
use crate::fixture::public;
use nonos_bt_bond::{BondError, BondRecord, Keys, KeysError, Store, KIND_BOND, MAX_BONDS};
use nonos_bt_wire::addr::{AddrType, BdAddr};

fn kept_ltk(s: &Store<Faulty>) -> Option<[u8; 16]> {
    s.find(AddrType::Public, &BdAddr(public().identity)).map(|r| r.ltk)
}

fn fresh() -> BondRecord {
    BondRecord { ltk: *b"a-fresh-ltk-here", ..public() }
}

#[test]
fn a_refused_store_keeps_the_old_bond() {
    let mut s = Store::open(Faulty::default()).unwrap();
    s.keep(public()).unwrap();
    s.keys().store = Some(KeysError::Full);
    assert_eq!(s.keep(fresh()), Err(BondError::Keys(KeysError::Full)));
    assert_eq!(kept_ltk(&s), Some(public().ltk));
    assert_eq!(s.keys().inner.list(KIND_BOND).unwrap().len(), 1);
}

#[test]
fn a_delete_and_rollback_both_refused_keep_the_old_bond() {
    let mut s = Store::open(Faulty::default()).unwrap();
    s.keep(public()).unwrap();
    s.keys().delete = Some(KeysError::Denied);
    assert_eq!(s.keep(fresh()), Err(BondError::Keys(KeysError::Denied)));
    assert_eq!(kept_ltk(&s), Some(public().ltk));
    s.keys().delete = None;
    // The rollback was refused too, so both records remain; the next keep
    // replaces the one the book knows, and the stray is read back as a
    // second copy of the same identity, which the book holds once.
    s.keep(fresh()).unwrap();
    let s = Store::open(s.into_keys()).unwrap();
    assert_eq!(s.book().bonds().len(), 1);
}

#[test]
fn a_full_book_stores_nothing_new_but_renews_a_bond() {
    let mut s = Store::open(Faulty::default()).unwrap();
    for i in 0..MAX_BONDS as u8 {
        s.keep(BondRecord { identity: [i, 9, 9, 9, 9, 9], ..public() }).unwrap();
    }
    let stores = s.keys().stores;
    let new = BondRecord { identity: [0xee, 9, 9, 9, 9, 9], ..public() };
    assert_eq!(s.keep(new), Err(BondError::Full));
    assert_eq!(s.keys().stores, stores);
    s.keep(BondRecord { identity: [3, 9, 9, 9, 9, 9], ..fresh() }).unwrap();
    assert_eq!(s.keys().inner.list(KIND_BOND).unwrap().len(), MAX_BONDS);
}
