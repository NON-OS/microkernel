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

//! Which table a disk names, from its first two sectors, its last, and the
//! arrays they point at (UEFI 2.10, 5.3.2, as recalled, unverified). The
//! primary when it and its array hold, the backup when only it and its
//! array do. Either is only read: repairing the other is the owner's tool's
//! work, never the kernel's. Pure but for the array reads it is handed.

use alloc::vec::Vec;

use super::entry::entries;
use super::header::{parse_as, Header, SIGNATURE};
use super::mbr::protects;
pub(crate) use super::table::{Held, Table};
use super::which::Which;

/// The table on a disk of `sectors` whose LBA 0, LBA 1 and last LBA are
/// `mbr`, `first` and `last`; `array` reads the array a header names.
pub(crate) fn choose<E>(
    (mbr, first, last): (&[u8], &[u8], &[u8]),
    sectors: u64,
    mut array: impl FnMut(&Header) -> Result<Vec<u8>, E>,
) -> Result<Table, E> {
    if !protects(mbr) {
        let signed = first.get(..8) == Some(&SIGNATURE[..]);
        return Ok(if signed { Table::Unbelieved } else { Table::NoGpt });
    }
    let primary = parse_as(first, sectors, Which::Primary);
    let backup = parse_as(last, sectors, Which::Backup);
    if let Some(p) = primary {
        if let Some(found) = entries(&p, &array(&p)?) {
            let differ = backup.is_some_and(|b| !agree(&p, &b));
            return Ok(Table::Named(found, if differ { Held::Disagreeing } else { Held::Primary }));
        }
    }
    if let Some(b) = backup {
        if let Some(found) = entries(&b, &array(&b)?) {
            return Ok(Table::Named(found, Held::Backup));
        }
    }
    Ok(Table::Unbelieved)
}

/// The two headers name one disk, one usable range and one array.
fn agree(p: &Header, b: &Header) -> bool {
    let range = |h: &Header| (h.disk_guid, h.first_usable, h.last_usable, h.entries_crc);
    range(p) == range(b)
}
