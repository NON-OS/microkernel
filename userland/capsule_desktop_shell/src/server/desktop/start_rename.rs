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

//! Begin an inline rename of the desktop item at `index`, seeding the edit
//! buffer with its current name.

use crate::state::Context;

/// KEY_DOWN kind bit, grabbed so typed keys reach the shell during the rename
/// even though the desktop is not a focusable window.
pub fn start_rename(ctx: &mut Context, index: usize) {
    let name = match ctx.desktop_items.get(index) {
        Some(item) => item.name.clone(),
        None => return,
    };
    ctx.rename_buf = name;
    ctx.rename = Some(index);
    crate::server::grabs::sync(ctx);
}
