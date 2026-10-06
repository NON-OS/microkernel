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

//! Delete a desktop item: the menu asks (state::delete_prompt), the prompt's
//! Delete button removes it from the home directory and re-syncs.

use super::home::home_path;
use crate::state::Context;

/// The menu's Delete: put the question up for the item at `index`.
pub fn ask_delete(ctx: &mut Context, index: usize) {
    if let Some(item) = ctx.desktop_items.get(index) {
        ctx.pending_delete.ask(&item.name, item.is_dir);
    }
}

/// Remove `name` from the desktop's directory, once the prompt said Delete.
pub fn delete_entry(ctx: &mut Context, name: &str, is_dir: bool) {
    let Some(path) = home_path(name) else { return };
    match crate::vfs_client::remove(path.as_bytes(), is_dir) {
        Ok(()) => {
            let _ = super::refresh::refresh(ctx);
        }
        // A refused delete left the icon in place with no explanation.
        Err(code) => super::say::refused(ctx, b"Could not delete: ", code),
    }
}
