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

//! BONDS and FORGET, Settings' alone. A bond's keys never leave the
//! capsule: the list carries the identity and how the bond was made.

use super::board::Board;
use super::ops::{E_INVAL, E_IO, E_NOENT, E_OK};
use super::part::Identity;
use super::wire::{BOND_LEN, DATA_AT};
use nonos_bt_bond::Keys;

/// BONDS: a count, then for each bond in the list's order its identity
/// address type, the address (least significant octet first), 1 when
/// pairing was authenticated, and 1 when the link runs on it.
pub fn bonds<K>(b: &Board<K>, tx: &mut [u8]) -> usize {
    let list = b.bonds.book().bonds();
    for (i, r) in list.iter().enumerate() {
        let at = DATA_AT + 1 + BOND_LEN * i;
        let linked = b.current == Some((r.identity_type, r.identity));
        tx[at] = r.identity_type;
        tx[at + 1..at + 7].copy_from_slice(&r.identity);
        tx[at + 7] = u8::from(r.authenticated);
        tx[at + 8] = u8::from(linked);
    }
    tx[DATA_AT] = list.len() as u8;
    1 + BOND_LEN * list.len()
}

/// FORGET: the identity address type and the address, as BONDS gives them.
/// The bond is dropped from the store, its levels with it, and a link that
/// runs on it is ended.
pub fn forget<K: Keys>(b: &mut Board<K>, body: &[u8]) -> i32 {
    let Some(who) = identity(body) else { return E_INVAL };
    match b.bonds.forget(who.0, &who.1) {
        Ok(true) => {}
        Ok(false) => return E_NOENT,
        Err(_) => return E_IO,
    }
    b.levels.retain(|l| l.who != Some(who));
    if b.current == Some(who) {
        b.drop_link = true;
    }
    E_OK
}

fn identity(body: &[u8]) -> Option<Identity> {
    let [kind, a @ ..] = body else { return None };
    let addr: [u8; 6] = a.try_into().ok()?;
    (*kind <= 1).then_some((*kind, addr))
}
