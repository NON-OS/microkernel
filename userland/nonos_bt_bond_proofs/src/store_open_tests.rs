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

//! A keyring holding more bonds than the book: the book fills and the rest
//! are counted, not dropped from the keyring.

use crate::fixture::public;
use nonos_bt_bond::{BondRecord, Keys, MemoryKeys, Store, KIND_BOND, MAX_BONDS};

#[test]
fn bonds_past_the_book_are_counted_and_left_kept() {
    let mut keys = MemoryKeys::default();
    for i in 0..=MAX_BONDS as u8 {
        let record = BondRecord { identity: [i, 7, 7, 7, 7, 7], ..public() };
        keys.store(KIND_BOND, &record.encode().unwrap()).unwrap();
    }
    let mut s = Store::open(keys).unwrap();
    assert_eq!((s.book().bonds().len(), s.refused()), (MAX_BONDS, 1));
    assert_eq!(s.keys().list(KIND_BOND).unwrap().len(), MAX_BONDS + 1);
}
