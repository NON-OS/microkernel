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

//! The requester ids an I/O APIC or HPET sends its interrupts under (48882,
//! 5.2.2.2: SPECIAL 48h, the id at offset 5, recalled). A PCI function given
//! the same id would share its device table entry, and so its remapping
//! table, with that interrupt source (L50). Pure, over the table's bytes.

use super::ivhd_entry::{size_at, word};
use super::ivhd_scope::{block_at, FIRST_BLOCK};

/// The special devices' ids into `out`, once each: how many, and whether
/// any did not fit.
pub fn specials(table: &[u8], out: &mut [u16]) -> (usize, bool) {
    let (mut n, mut at, mut lost) = (0, FIRST_BLOCK, false);
    while let Some((bytes, next)) = block_at(table, at) {
        let mut e = 0;
        while let Some(len) = size_at(bytes, e) {
            let id = (bytes[e] == 0x48 && len >= 8).then(|| word(bytes, e + 5));
            match id.filter(|id| !out[..n].contains(id)) {
                Some(id) if n < out.len() => (out[n], n) = (id, n + 1),
                Some(_) => lost = true,
                None => {}
            }
            e += len;
        }
        at = next;
    }
    (n, lost)
}
