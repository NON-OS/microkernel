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
//! The devices discovery is lied to over, their HID services' handle
//! ranges, and the lies.

use super::rng::Rng;
use crate::map::maps::KEYBOARD;
use crate::map::mice::mouse;
use crate::sim::pointer_devices::keyboard_pointer;
use crate::sim::report_devices::report_keyboard;
use crate::sim::{combo, keyboard_mouse, Db};

pub fn device(k: usize) -> Db {
    match k {
        0 => keyboard_mouse(),
        1 => combo(),
        2 => report_keyboard(&KEYBOARD),
        _ => keyboard_pointer(&[&KEYBOARD[..], &mouse()].concat(), &[3, 1]),
    }
}

/// Each HID service's first and last handle, from the database itself.
pub fn services(db: &Db) -> Vec<(u16, u16)> {
    let decls: Vec<_> = db.attrs.iter().filter(|a| a.kind == [0x00, 0x28]).collect();
    let end = |i: usize| decls.get(i + 1).map_or(0xffff, |d| d.handle - 1);
    (0..decls.len())
        .filter(|&i| decls[i].value == [0x12, 0x18])
        .map(|i| (decls[i].handle, end(i)))
        .collect()
}

pub fn lie(rng: &mut Rng, a: &mut Vec<u8>) {
    match rng.below(4) {
        0 if !a.is_empty() => {
            let at = rng.below(a.len());
            a[at] ^= 1 + rng.below(255) as u8;
        }
        1 => a.truncate(rng.below(a.len().max(1))),
        2 => a.extend(rng.span(1, 30)),
        _ if a.len() >= 3 => {
            let at = 1 + rng.below(a.len() - 2);
            a[at..at + 2].copy_from_slice(&(rng.next() as u16).to_le_bytes());
        }
        _ => a.clear(),
    }
}
