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

//! A market program is never startable outside its box at its installed
//! path: held from before its bytes land until its policy is bound
//! (take_tests.rs: and still held when an uninstall leaves it). The store
//! here is a map the closures write.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::install::market::minted::{beside, place, PENDING, POLICY};

const ELF: &[u8] = b"\x7fELF a program";
const AT: &[u8] = b"/usr/bin/age";

type Store = BTreeMap<Vec<u8>, Vec<u8>>;

/// Place one file into `store`, every write in `order`, `refuse` unwritable.
fn placed(body: &[u8], held: bool, refuse: &[u8]) -> (bool, Store, Vec<Vec<u8>>) {
    let (mut store, mut order) = (BTreeMap::new(), Vec::new());
    let mut write = |at: &[u8], bytes: &[u8]| {
        order.push(at.to_vec());
        at != refuse && store.insert(at.to_vec(), bytes.to_vec()).is_none()
    };
    let ok = place(AT, body, held, &mut write);
    (ok, store, order)
}

#[test]
fn a_held_program_lands_after_the_word_that_holds_it() {
    let (ok, store, order) = placed(ELF, true, b"");
    assert!(ok);
    assert_eq!(order, [beside(AT, POLICY), AT.to_vec()], "held first, then the program");
    assert_eq!(store.get(&beside(AT, POLICY)).map(Vec::as_slice), Some(PENDING));
}

#[test]
fn a_program_that_cannot_be_held_is_not_written() {
    let (ok, store, _) = placed(ELF, true, &beside(AT, POLICY));
    assert!(!ok && !store.contains_key(AT));
}

#[test]
fn only_a_held_package_s_programs_are_held() {
    assert_eq!(placed(ELF, false, b"").2, [AT.to_vec()]);
    assert_eq!(placed(b"text", true, b"").2, [AT.to_vec()]);
}
