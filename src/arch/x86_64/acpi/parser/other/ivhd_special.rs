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

//! IVHD special entries (48h, 48882 5.2.2.2): each I/O APIC and HPET by its
//! handle (an I/O APIC's is the id the MADT gives) and the requester id its
//! interrupts arrive under. An I/O APIC's remapping table is keyed by that
//! id, and a PCI function given the same id would share its entry, and
//! so its remapping table, with that source (L50). Pure, over the bytes.

use super::ivhd_entry::{size_at, word};
use super::ivhd_scope::{block_at, FIRST_BLOCK};

pub const IOAPIC: u8 = 1;
pub const HPET: u8 = 2;
pub const MAX_SPECIALS: usize = 16;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Special {
    pub variety: u8,
    pub handle: u8,
    pub id: u16,
}

/// Every whole special entry, in table order.
fn each(table: &[u8], mut found: impl FnMut(Special)) {
    let mut at = FIRST_BLOCK;
    while let Some((bytes, next)) = block_at(table, at) {
        let mut e = 0;
        while let Some(len) = size_at(bytes, e) {
            if bytes[e] == 0x48 && len >= 8 {
                let (variety, handle) = (bytes[e + 7], bytes[e + 4]);
                found(Special { variety, handle, id: word(bytes, e + 5) });
            }
            e += len;
        }
        at = next;
    }
}

/// The special devices' ids into `out`, once each: how many, and whether any did not fit.
pub fn specials(table: &[u8], out: &mut [u16]) -> (usize, bool) {
    let (mut n, mut lost) = (0, false);
    each(table, |s| match Some(s.id).filter(|id| !out[..n].contains(id)) {
        Some(id) if n < out.len() => (out[n], n) = (id, n + 1),
        Some(_) => lost = true,
        None => {}
    });
    (n, lost)
}

/// The special entries into `out` in table order: how many. A 10h and an 11h
/// block naming the same one count once; past `out`'s room the rest are not kept.
pub fn special_entries(table: &[u8], out: &mut [Special]) -> usize {
    let mut n = 0;
    each(table, |s| {
        if !out[..n].contains(&s) && n < out.len() {
            out[n] = s;
            n += 1;
        }
    });
    n
}
