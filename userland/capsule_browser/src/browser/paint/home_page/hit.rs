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

use crate::browser::omnibox::shortcut_at;
use crate::browser::paint::home_page::shortcut_data::SHORTCUTS;

/* The site of the shortcut badge under (x, y) on a home page `width` pixels
 * wide, the width it was painted at. */
pub fn shortcut_url_at(x: i32, y: i32, width: u32) -> Option<&'static str> {
    let i = shortcut_at(x, y, width, SHORTCUTS.len() as u32)?;
    Some(SHORTCUTS[i].url)
}
