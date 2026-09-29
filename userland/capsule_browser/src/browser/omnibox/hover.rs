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

/* Record the link under the pointer. True when it changed, which is the
 * only time the status bubble needs drawing again. */
pub fn hover_update(slot: &mut Option<String>, next: Option<&str>) -> bool {
    if slot.as_deref() == next {
        return false;
    }
    *slot = next.map(String::from);
    true
}

/* Each navigation takes a new generation; a load's result is committed only
 * while its generation is still current. A result that arrives after Stop,
 * Home or a newer navigation belongs to a page the reader left. */
pub fn should_commit(nav_gen: u32, loading_gen: u32) -> bool {
    nav_gen == loading_gen
}
