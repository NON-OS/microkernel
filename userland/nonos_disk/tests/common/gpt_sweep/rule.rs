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

//! Which table a disk names, stated apart from the reader (UEFI 2.10 5.3.2,
//! as recalled, unverified): none behind no protective MBR, the primary
//! when it and its array hold, else the backup when it and its array hold;
//! a disk showing a GPT with neither is a GPT unbelieved.

use super::disk::Disk;
use super::model::{header, protects, table, Part};

#[derive(Debug, PartialEq, Eq)]
pub enum Named {
    NoGpt,
    Unbelieved,
    Primary(Vec<Part>),
    Backup(Vec<Part>),
    /// The primary, its backup holding but naming another disk, usable
    /// range or array.
    Disagreeing(Vec<Part>),
}

pub fn rule(d: &Disk) -> Named {
    if !protects(&d.mbr) {
        return if &d.heads[0][..8] == b"EFI PART" { Named::Unbelieved } else { Named::NoGpt };
    }
    if let Some(p) = table(d, 0) {
        let names = |w: usize| {
            (d.heads[w][56..72].to_vec(), d.get(w, 40, 8), d.get(w, 48, 8), d.get(w, 88, 4))
        };
        return match header(d, 1).is_some() && names(0) != names(1) {
            true => Named::Disagreeing(p),
            false => Named::Primary(p),
        };
    }
    table(d, 1).map_or(Named::Unbelieved, Named::Backup)
}
