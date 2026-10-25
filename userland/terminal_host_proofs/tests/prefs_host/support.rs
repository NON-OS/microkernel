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

//! Field-wise comparison and display of a record.

use crate::term::prefs::types::Prefs;

pub fn same(a: &Prefs, b: &Prefs) -> bool {
    a.theme == b.theme
        && a.font_scale == b.font_scale
        && a.cursor == b.cursor
        && a.rails == b.rails
        && a.project_slice() == b.project_slice()
}

pub fn show(p: &Prefs) -> String {
    format!(
        "theme {} font_scale {} cursor {} rails {:#04b}",
        p.theme, p.font_scale, p.cursor, p.rails
    )
}
