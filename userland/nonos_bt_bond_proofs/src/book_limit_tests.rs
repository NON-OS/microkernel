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

//! Keeping bonds: one per identity, renewed in place, at most fifteen.

use crate::book_tests::book;
use crate::fixture::public;
use nonos_bt_bond::{BondError, BondRecord, Book, MAX_BONDS};
use nonos_bt_wire::addr::{AddrType, BdAddr};

#[test]
fn a_new_bond_replaces_the_old_one_for_the_same_identity() {
    let mut b = book();
    b.insert(BondRecord { ltk: *b"a-fresh-ltk-here", ..public() }).unwrap();
    assert_eq!(b.bonds().len(), 2);
    let found = b.find(AddrType::Public, &BdAddr(public().identity)).unwrap();
    assert_eq!(found.ltk, *b"a-fresh-ltk-here");
    assert!(b.remove(0, &public().identity));
    assert!(!b.remove(0, &public().identity));
    assert_eq!(b.bonds().len(), 1);
}

#[test]
fn the_book_holds_fifteen_valid_bonds_and_still_renews_one() {
    let mut b = Book::default();
    assert_eq!(b.insert(BondRecord { key_size: 3, ..public() }), Err(BondError::Invalid));
    let nth = |i: u8, ltk| BondRecord { identity: [i, 0, 0, 0, 0, 0], ltk, ..public() };
    for i in 0..MAX_BONDS as u8 {
        b.insert(nth(i, public().ltk)).unwrap();
    }
    assert_eq!(b.insert(nth(0xee, public().ltk)), Err(BondError::Full));
    b.insert(nth(3, *b"replaced-ltk-abc")).unwrap();
    assert_eq!(b.bonds().len(), MAX_BONDS);
    assert_eq!(
        b.find(AddrType::Public, &BdAddr([3, 0, 0, 0, 0, 0])).unwrap().ltk[..8],
        *b"replaced"
    );
}
