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

//! This host's IRK, drawn once and kept, the same in every later boot.

use crate::faulty::Faulty;
use nonos_bt_bond::{BondError, Keys, KeysError, LocalIrk, MemoryKeys, Store, KIND_LOCAL_IRK};

const KEY: [u8; 16] = *b"this-hosts-irk!!";

#[test]
fn the_key_is_drawn_once_and_read_back_after() {
    let mut s = Store::open(MemoryKeys::default()).unwrap();
    assert_eq!(s.local_irk(|| KEY), Ok(LocalIrk::new(KEY).unwrap()));
    let again = s.local_irk(|| panic!("drawn twice"));
    assert_eq!(again.unwrap().key(), &KEY);
    let mut s = Store::open(s.into_keys()).unwrap();
    assert_eq!(s.local_irk(|| panic!("drawn after a restart")).unwrap().key(), &KEY);
    assert_eq!(s.keys().list(KIND_LOCAL_IRK).unwrap().len(), 1);
}

#[test]
fn a_zero_draw_is_refused_and_nothing_is_kept() {
    let mut s = Store::open(MemoryKeys::default()).unwrap();
    assert_eq!(s.local_irk(|| [0; 16]), Err(BondError::Invalid));
    assert!(s.keys().list(KIND_LOCAL_IRK).unwrap().is_empty());
}

#[test]
fn an_unreadable_key_is_passed_over_and_a_refused_store_is_an_error() {
    let mut keys = Faulty::default();
    keys.inner.store(KIND_LOCAL_IRK, &[1, 2, 3]).unwrap();
    let mut s = Store::open(keys).unwrap();
    s.keys().store = Some(KeysError::Full);
    assert_eq!(s.local_irk(|| KEY), Err(BondError::Keys(KeysError::Full)));
    s.keys().store = None;
    assert_eq!(s.local_irk(|| KEY).unwrap().key(), &KEY);
}
