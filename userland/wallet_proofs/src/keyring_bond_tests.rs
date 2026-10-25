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

//! Records kept by name (L33): a Bluetooth bond outlives the pid that stored
//! it, goes to whoever holds the name next, and to nobody else.

use crate::bond_gate::{holder_of, BOND_HOLDERS};
use crate::store::{KeyType, Store, StoreError, MAX_KEYS_PER_OWNER};

const BT: u8 = 0;
const OLD: u32 = 80;
const NEW: u32 = 81;
const OTHER: u32 = 90;
const BOND: [u8; 48] = [0x5A; 48];

fn bond(s: &mut Store, pid: u32) -> Result<u32, StoreError> {
    s.store_named(BT, KeyType::BtBond, &BOND, pid, 0, 0)
}

/// The id a store gave; StoreError has no Debug to unwrap with.
fn given(r: Result<u32, StoreError>) -> u32 {
    match r {
        Ok(id) => id,
        Err(_) => panic!("the store refused"),
    }
}

fn reads(s: &mut Store, id: u32, pid: u32) -> bool {
    matches!(s.retrieve(id, pid), Ok(bytes) if bytes == BOND)
}

#[test]
fn a_bond_comes_only_through_the_holders_door() {
    let mut s = Store::new();
    assert!(matches!(s.store(KeyType::BtBond, &BOND, OTHER, 0, 0), Err(StoreError::AccessDenied)));
    assert!(matches!(
        s.store(KeyType::BtLocalIrk, &[1; 17], OTHER, 0, 0),
        Err(StoreError::AccessDenied)
    ));
    assert!(matches!(
        s.store_named(BT, KeyType::Symmetric, &BOND, OLD, 0, 0),
        Err(StoreError::InvalidArgument)
    ));
    let id = given(bond(&mut s, OLD));
    assert!(reads(&mut s, id, OLD));
    assert!(matches!(s.retrieve(id, OTHER), Err(StoreError::AccessDenied)));
}

#[test]
fn a_bond_outlives_its_driver_and_goes_to_the_next_holder() {
    let mut s = Store::new();
    let id = given(bond(&mut s, OLD));
    let plain = given(s.store(KeyType::Symmetric, &BOND, OLD, 0, 0));
    assert_eq!(s.drop_ended(|_| false), 1, "only the plain key goes with the pid");
    assert!(matches!(s.retrieve(plain, OLD), Err(StoreError::NotFound)));
    assert!(s.awaits_holder(NEW));
    assert!(matches!(s.retrieve(id, NEW), Err(StoreError::AccessDenied)), "not before adoption");
    assert_eq!(s.adopt(BT, NEW), 1);
    assert!(!s.awaits_holder(NEW));
    assert!(reads(&mut s, id, NEW));
    assert!(matches!(s.retrieve(id, OLD), Err(StoreError::AccessDenied)));
    assert_eq!(s.ids_owned_by(NEW, KeyType::BtBond), [id]);
}

#[test]
fn a_list_names_only_the_callers_own_kind() {
    let mut s = Store::new();
    let a = given(bond(&mut s, OLD));
    let irk = given(s.store_named(BT, KeyType::BtLocalIrk, &[2; 17], OLD, 0, 0));
    let b = given(bond(&mut s, OLD));
    given(s.store(KeyType::Symmetric, &BOND, OTHER, 0, 0));
    assert_eq!(s.ids_owned_by(OLD, KeyType::BtBond), [a, b]);
    assert_eq!(s.ids_owned_by(OLD, KeyType::BtLocalIrk), [irk]);
    assert!(s.ids_owned_by(OTHER, KeyType::BtBond).is_empty());
}

#[test]
fn a_restarted_holder_keeps_one_share() {
    let mut s = Store::new();
    while bond(&mut s, OLD).is_ok() {}
    assert_eq!(s.ids_owned_by(OLD, KeyType::BtBond).len(), MAX_KEYS_PER_OWNER);
    // The new pid's store adopts the old records first and finds the share full.
    assert!(matches!(bond(&mut s, NEW), Err(StoreError::Full)));
    assert_eq!(s.ids_owned_by(NEW, KeyType::BtBond).len(), MAX_KEYS_PER_OWNER);
}

#[test]
fn only_the_names_owner_is_a_holder() {
    assert_eq!(BOND_HOLDERS[0], b"driver.bt_usb0");
    let owner = |name: &[u8]| (name == b"driver.bt_usb0").then_some(NEW);
    assert_eq!(holder_of(NEW, owner), Some(BT));
    assert_eq!(holder_of(OLD, owner), None);
    assert_eq!(holder_of(0, |_| Some(0)), None, "pid 0 is never a holder");
    assert_eq!(holder_of(NEW, |_| None), None, "an unregistered name has no holder");
}
