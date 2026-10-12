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

//! Which requester ids reach an AMD IOMMU under another id (48882, 5.2.2.2:
//! ALIAS SELECT 42h, or ALIAS START 43h to the next END 04h). Their
//! interrupts arrive on the alias's entry, so a table on their own would
//! never see them (L47). Pure, over the table's bytes.

use super::ivhd_entry::{size, word};
use super::ivhd_scope::{block_at, FIRST_BLOCK};

pub const MAX_ALIASES: usize = 32;

/// Requester ids `first..=last` arrive as `alias`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Alias {
    pub first: u16,
    pub last: u16,
    pub alias: u16,
}

/// The aliases into `out`: how many, and whether any did not fit.
pub fn aliases(table: &[u8], out: &mut [Alias]) -> (usize, bool) {
    let (mut n, mut at, mut lost) = (0, FIRST_BLOCK, false);
    while let Some((bytes, next)) = block_at(table, at) {
        let (mut open, mut e) = (None, 0);
        while let Some(len) = size_at(bytes, e) {
            let (id, alias) = (word(bytes, e + 1), if len >= 8 { word(bytes, e + 5) } else { 0 });
            let found = match bytes[e] {
                0x42 => Some(Alias { first: id, last: id, alias }),
                0x43 => {
                    open = Some((id, alias));
                    None
                }
                // A plain range start ends any aliased one left open.
                0x03 | 0x47 => {
                    open = None;
                    None
                }
                0x04 => open.take().map(|(first, alias)| Alias { first, last: id, alias }),
                _ => None,
            };
            match found.filter(|a| a.first <= a.last) {
                Some(a) if n < out.len() => (out[n], n) = (a, n + 1),
                Some(_) => lost = true,
                None => {}
            }
            e += len;
        }
        at = next;
    }
    (n, lost)
}

fn size_at(bytes: &[u8], at: usize) -> Option<usize> {
    (at + 4 <= bytes.len()).then(|| size(bytes, at)).flatten().filter(|len| at + len <= bytes.len())
}

/// Whether `id`'s requests arrive under another id.
pub fn aliased(list: &[Alias], id: u16) -> bool {
    list.iter().any(|a| a.first <= id && id <= a.last && a.alias != id)
}
