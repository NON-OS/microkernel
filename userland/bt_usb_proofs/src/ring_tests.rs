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

//! Bonds kept in the keyring through the capsule's client, as a capsule
//! that stops and starts again finds them.

use crate::ring_sim::{capsule, ring};
use nonos_bt_bond::{BondError, BondRecord, Keys, KeysError, Store, KIND_BOND};
use nonos_bt_wire::addr::{AddrType, BdAddr};

pub fn record(last: u8) -> BondRecord {
    BondRecord {
        authenticated: true,
        identity_type: 1,
        identity: [1, 2, 3, 4, 5, 0xc0 | last],
        ltk: [0x11; 16],
        irk: Some([0x22; 16]),
        key_size: 16,
    }
}

#[test]
fn a_restarted_capsule_finds_its_bonds_and_its_irk_again() {
    let ring = ring(true);
    let mut store = Store::open(capsule(&ring, 7)).unwrap();
    store.keep(record(1)).unwrap();
    let irk = store.local_irk(|| [9; 16]).unwrap();
    drop(store);
    let mut again = Store::open(capsule(&ring, 7)).unwrap();
    let found = again.find(AddrType::Random, &BdAddr([1, 2, 3, 4, 5, 0xc1]));
    assert_eq!(found, Some(&record(1)));
    assert_eq!(again.local_irk(|| panic!("drawn again")).unwrap(), irk);
    assert_eq!(again.refused(), 0);
}

#[test]
fn another_capsules_records_are_neither_listed_nor_read() {
    let ring = ring(true);
    let mut other = Store::open(capsule(&ring, 8)).unwrap();
    other.keep(record(2)).unwrap();
    let mut ours = capsule(&ring, 7);
    assert_eq!(ours.list(KIND_BOND), Ok(vec![]));
    let theirs = *ring.borrow().records.keys().next().unwrap();
    assert_eq!(ours.retrieve(theirs), Err(KeysError::Denied));
    assert_eq!(ours.delete(theirs), Err(KeysError::Denied));
    assert_eq!(ring.borrow().records.len(), 1);
}

#[test]
fn a_keyring_without_l33_opens_no_store() {
    let ring = ring(false);
    let opened = Store::open(capsule(&ring, 7)).err();
    assert_eq!(opened, Some(BondError::Keys(KeysError::Unavailable)));
}
