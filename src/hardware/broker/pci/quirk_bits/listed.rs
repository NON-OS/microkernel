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

//! The standard capability list from 0x34 (PCI Local Bus 3.0 section 6.7,
//! as recalled, unverified), and how far each entry on it reaches. EHCI's
//! legacy support capability is not on it (EECP names it), but the PCI
//! Power Management capability shares its id, 01h, so a rule finding
//! legacy support by that id must first rule out every listed span.

use super::span::span;

const STATUS: u16 = 0x06;
const HAS_LIST: u8 = 0x10;
const LIST_POINTER: u16 = 0x34;
const FLOOR: u16 = 0x40;
const END: u16 = 0x100;
/// The most entries the 192 bytes above the header can hold, 4 each; a
/// longer or looped list stops there.
const MOST: usize = 48;

/// Whether config byte `at` lies inside a capability on the standard list
/// of the function `read` reads.
pub(super) fn in_listed(read: &dyn Fn(u16) -> u8, at: u16) -> bool {
    let (starts, n) = starts(read);
    let starts = &starts[..n];
    starts.iter().any(|&s| {
        let next = starts.iter().copied().filter(|&o| o > s).min().unwrap_or(END);
        let end = span(read, s).map_or(next, |len| s + len);
        (s..end).contains(&at)
    })
}

/// The list's entries, in order, each read once: a pointer under the
/// header or back to an entry already seen ends it.
fn starts(read: &dyn Fn(u16) -> u8) -> ([u16; MOST], usize) {
    let mut out = [0u16; MOST];
    let mut n = 0;
    if read(STATUS) & HAS_LIST == 0 {
        return (out, 0);
    }
    let mut at = (read(LIST_POINTER) & 0xFC) as u16;
    while at >= FLOOR && n < MOST && !out[..n].contains(&at) {
        out[n] = at;
        n += 1;
        at = (read(at + 1) & 0xFC) as u16;
    }
    (out, n)
}
