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

//! How many cursor stops each row and each section has.

use nonos_policy_proto::wallpaper_labels::WALLPAPER_LABELS;

use crate::settings::section::Section;

use super::blocks_for::blocks_for;
use super::rows::Row;

/// How many rows the cursor stops on in a section. The cursor indexes this
/// list, and the pane walks the same block tables, so the two cannot disagree
/// about which row is which.
pub fn field_count(section: Section) -> usize {
    let mut n = 0;
    for b in blocks_for(section) {
        for r in b.rows {
            n += stops(r);
        }
    }
    n
}

/// How many cursor stops a row is.
pub fn stops(r: &Row) -> usize {
    match r {
        Row::Field(_) | Row::Access(_) => 1,
        Row::Wallpapers => WALLPAPER_LABELS.len(),
        _ => 0,
    }
}

impl Row {
    /// A row the cursor stops on by itself (the wallpapers and networks are
    /// laid out, and stopped on, a row each).
    pub fn is_stop(&self) -> bool {
        matches!(self, Row::Field(_) | Row::Access(_))
    }
}
