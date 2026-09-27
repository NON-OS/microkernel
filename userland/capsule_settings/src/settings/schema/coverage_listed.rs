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

//! The other direction: a row may only name a field in `ALL_FIELDS`, the list
//! of fields that some code reads, so a switch wired to nothing fails the build.

use crate::settings::section::{SECTIONS, SECTION_COUNT};

use super::all_fields::ALL_FIELDS;
use super::blocks_for::blocks_for;
use super::rows::Row;

const fn listed(id: u32) -> bool {
    let mut i = 0;
    while i < ALL_FIELDS.len() {
        if ALL_FIELDS[i] as u32 == id {
            return true;
        }
        i += 1;
    }
    false
}

// Every row on a screen names a field from the list of fields with a reader.
const fn all_listed() -> bool {
    let mut s = 0;
    while s < SECTION_COUNT {
        let blocks = blocks_for(SECTIONS[s]);
        let mut b = 0;
        while b < blocks.len() {
            let mut r = 0;
            while r < blocks[b].rows.len() {
                if let Row::Field(f) = blocks[b].rows[r] {
                    if !listed(f as u32) {
                        return false;
                    }
                }
                r += 1;
            }
            b += 1;
        }
        s += 1;
    }
    true
}

const _: () = assert!(all_listed(), "a settings row must name a field that something reads");
