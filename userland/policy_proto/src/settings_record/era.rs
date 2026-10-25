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

//! The wallpaper catalog's era in the settings record.
//!
//! The record names the desktop wallpaper and the wallpapers kept by catalog
//! index, and an index means a picture only within one era of the catalog
//! (`crate::wallpaper_labels::CATALOG_ERA`). So the record says its era, as
//! an entry under an id that no field has: a build that does not know the id
//! skips the entry and restores everything else, as it would any field it does
//! not keep. A record without this era's entry was written by a build whose
//! indices name other pictures, and its two wallpaper entries are not taken:
//! the store's default stands until the person picks again.

use crate::field::Field;
use crate::kind::KIND_U8;
use crate::wallpaper_labels::CATALOG_ERA;

/// The entry's id, outside every field's (0x0101 to 0x03FF).
pub(super) const ID: u16 = 0x7F01;

pub(super) const ENTRY_LEN: usize = 5;

/// The entry `encode` writes first.
pub(super) const ENTRY: [u8; ENTRY_LEN] = [ID as u8, (ID >> 8) as u8, KIND_U8, 1, CATALOG_ERA];

/// Whether `entries`, a record past its magic, carries this era's entry.
pub(super) fn is_this(mut entries: &[u8]) -> bool {
    while let Some(head) = entries.get(..4) {
        let len = head[3] as usize;
        let Some(body) = entries.get(4..4 + len) else { return false };
        if head[..3] == ENTRY[..3] && body == [CATALOG_ERA] {
            return true;
        }
        entries = &entries[4 + len..];
    }
    false
}

/// Whether `field` names a picture by its catalog index.
pub(super) fn names_a_picture(field: Field) -> bool {
    matches!(field, Field::Wallpaper | Field::WallpapersKept)
}
