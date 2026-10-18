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

//! The ops that only report: HEALTH, LINK and BATTERY.

use super::board::Board;
use super::ops::{NO_CONTROLLER, READY};
use super::part::Part;
use super::wire::{DATA_AT, VERSION};
use nonos_bt_bond::Store;
use nonos_bt_proto::{Health, Link, NO_PLACE};

/// HEALTH: the protocol's version, then 1 when a controller is up.
pub fn health<K>(b: &Board<K>, tx: &mut [u8]) -> usize {
    let h = Health { version: VERSION, controller: b.controller };
    tx[DATA_AT..DATA_AT + 3].copy_from_slice(&h.write());
    3
}

/// LINK: the state, a flags octet, and the link's bond as its place in the
/// bond list (0xFF for none). Flags: keyboard, mouse, keys post, scanning
/// on, pairing window open, pairing waiting on the user (bits 0 to 5).
pub fn link<K>(b: &Board<K>, now_ms: u64, tx: &mut [u8]) -> usize {
    let (k, m, keys) = b.ready.unwrap_or_default();
    let bits = [k, m, keys, b.scan, b.may_pair(now_ms), b.part != Part::None];
    let flags = bits.iter().enumerate().fold(0u8, |f, (i, &on)| f | u8::from(on) << i);
    let state = match (b.controller, b.ready) {
        (false, _) => NO_CONTROLLER,
        (true, Some(_)) => READY,
        (true, None) => b.state,
    };
    let place = b.current.map_or(NO_PLACE, |who| slot_of(&b.bonds, who));
    tx[DATA_AT..DATA_AT + 3].copy_from_slice(&Link { state, flags, place }.write());
    3
}

/// BATTERY: a count, then each level as the bond's place in the bond list
/// (0xFF for a link that kept no bond), the Battery Service's instance and
/// the percent. A level whose bond is gone is left out.
pub fn battery<K>(b: &Board<K>, tx: &mut [u8]) -> usize {
    let mut n = 0usize;
    for l in &b.levels {
        let slot = l.who.map_or(NO_PLACE, |who| slot_of(&b.bonds, who));
        if l.who.is_some() && slot == NO_PLACE {
            continue;
        }
        let at = DATA_AT + 1 + 3 * n;
        tx[at..at + 3].copy_from_slice(&[slot, l.instance, l.percent]);
        n += 1;
    }
    tx[DATA_AT] = n as u8;
    1 + 3 * n
}

/// A bond's place in the bond list, 0xFF when it has none.
pub fn slot_of<K>(bonds: &Store<K>, who: super::part::Identity) -> u8 {
    let at = bonds.book().bonds().iter().position(|r| (r.identity_type, r.identity) == who);
    at.map_or(NO_PLACE, |i| i as u8)
}
