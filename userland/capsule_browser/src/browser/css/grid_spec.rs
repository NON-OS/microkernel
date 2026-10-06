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

use alloc::string::String;
use alloc::vec::Vec;

/* Named grid data the Copy style struct cannot hold, kept in a per-node
 * side table like background images. Containers carry the named column
 * lines and template areas; items carry their requested placement. */
#[derive(Default)]
pub struct GridSpec {
    /* (name, zero-based column line index) from [name] groups in the
     * grid-template-columns track list. */
    pub col_lines: Vec<(String, u8)>,
    /* grid-template-areas rows, each a list of cell tokens ("." is a hole). */
    pub areas: Vec<Vec<String>>,
    /* grid-area: <name> on an item. */
    pub area: Option<String>,
    /* grid-column and grid-row lines as written: an integer (negative
     * counts from the end), a line or area name, span N, or auto. */
    pub col_start: Option<String>,
    pub col_end: Option<String>,
    pub row_start: Option<String>,
    pub row_end: Option<String>,
}

impl GridSpec {
    /* True when this node requests an explicit item placement. */
    pub fn places_item(&self) -> bool {
        self.area.is_some()
            || self.col_start.is_some()
            || self.col_end.is_some()
            || self.row_start.is_some()
            || self.row_end.is_some()
    }

    /* The node's spec, created on its first grid declaration. */
    pub(super) fn ensure(spec: &mut Option<GridSpec>) -> &mut GridSpec {
        spec.get_or_insert_with(GridSpec::default)
    }

    /* A grid line written as a custom identifier: not a number, auto or span. */
    pub fn is_ident(v: &str) -> bool {
        let first = v.bytes().next().unwrap_or(b'0');
        !(first.is_ascii_digit()
            || matches!(first, b'-' | b'+')
            || v == "auto"
            || v.starts_with("span"))
    }
}
