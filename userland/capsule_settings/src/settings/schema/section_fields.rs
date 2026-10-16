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

use nonos_policy_proto::Field;

use crate::settings::section::Section;

use super::access_knob::Knob;
use super::blocks_for::blocks_for;
use super::rows::Row;
use super::stops::stops;

/// A row the cursor stops on: a field, or one wallpaper of the collection.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    Field(Field),
    Wallpaper(u8),
    Access(Knob),
}

/// The row the cursor's `index` names.
pub fn slot_at(section: Section, index: usize) -> Option<Slot> {
    let mut n = 0;
    for b in blocks_for(section) {
        for r in b.rows {
            let k = stops(r);
            if index < n + k {
                return match r {
                    Row::Field(f) => Some(Slot::Field(*f)),
                    Row::Wallpapers => Some(Slot::Wallpaper((index - n) as u8)),
                    Row::Access(k) => Some(Slot::Access(*k)),
                    _ => None,
                };
            }
            n += k;
        }
    }
    None
}

/// The field the cursor's `index` names, if that row is a field.
pub fn field_at(section: Section, index: usize) -> Option<Field> {
    match slot_at(section, index)? {
        Slot::Field(f) => Some(f),
        Slot::Wallpaper(_) | Slot::Access(_) => None,
    }
}
