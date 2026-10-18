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

//! Bonds kept through the keyring's operations: read back by a restarted
//! capsule, renewed in place, forgotten, and never destroyed unread.

use crate::faulty::Faulty;
use crate::fixture::{public, with_irk};
use nonos_bt_bond::{BondRecord, Keys, MemoryKeys, Store, KIND_BOND};
use nonos_bt_wire::addr::{AddrType, BdAddr};

fn ltk_of(s: &Store<MemoryKeys>, b: &BondRecord) -> Option<[u8; 16]> {
    let kind = if b.identity_type == 0 { AddrType::Public } else { AddrType::Random };
    s.find(kind, &BdAddr(b.identity)).map(|r| r.ltk)
}

#[test]
fn a_restarted_capsule_finds_its_bonds_again() {
    let mut s = Store::open(MemoryKeys::default()).unwrap();
    s.keep(with_irk()).unwrap();
    s.keep(public()).unwrap();
    let s = Store::open(s.into_keys()).unwrap();
    assert_eq!((s.book().bonds().len(), s.refused()), (2, 0));
    assert_eq!(ltk_of(&s, &with_irk()), Some(with_irk().ltk));
    assert_eq!(ltk_of(&s, &public()), Some(public().ltk));
}

#[test]
fn a_bond_renewed_is_kept_once_and_a_bond_forgotten_is_gone() {
    let mut s = Store::open(MemoryKeys::default()).unwrap();
    s.keep(public()).unwrap();
    let fresh = BondRecord { ltk: *b"a-fresh-ltk-here", ..public() };
    s.keep(fresh.clone()).unwrap();
    assert_eq!(s.keys().list(KIND_BOND).unwrap().len(), 1);
    let mut s = Store::open(s.into_keys()).unwrap();
    assert_eq!(ltk_of(&s, &public()), Some(fresh.ltk));
    assert_eq!(s.forget(0, &public().identity), Ok(true));
    assert_eq!(s.forget(0, &public().identity), Ok(false));
    assert!(s.keys().list(KIND_BOND).unwrap().is_empty());
    assert!(s.book().bonds().is_empty());
}

#[test]
fn a_record_that_does_not_read_is_counted_and_left_alone() {
    let mut keys = MemoryKeys::default();
    let mut bad = with_irk().encode().unwrap();
    bad[0] = 9;
    keys.store(KIND_BOND, &bad).unwrap();
    keys.store(KIND_BOND, &public().encode().unwrap()).unwrap();
    let mut s = Store::open(keys).unwrap();
    assert_eq!((s.book().bonds().len(), s.refused()), (1, 1));
    assert_eq!(s.keys().list(KIND_BOND).unwrap().len(), 2);
}

#[test]
fn a_keyring_that_cannot_list_is_an_error_not_an_empty_book() {
    use nonos_bt_bond::{BondError, KeysError};
    let keys = Faulty { list: Some(KeysError::Unavailable), ..Faulty::default() };
    let err = Store::open(keys).err();
    assert_eq!(err, Some(BondError::Keys(KeysError::Unavailable)));
}
