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

//! Create a file or folder from the desktop right-click menu.

use super::home::home_path;
use crate::state::Context;

/// Create a new file or folder on the desktop (the home directory the icons
/// list) under a non-colliding name and pull the desktop back in sync. Repainting is left to the caller so it can first
/// tidy up its own state, such as closing the menu that triggered this.
pub fn create_entry(ctx: &mut Context, is_file: bool) {
    let base = if is_file { "New File" } else { "New Folder" };
    let name = super::unique_name::unique_name(ctx, base);
    let Some(path) = home_path(&name) else { return };
    let created = if is_file {
        crate::vfs_client::create_file(path.as_bytes())
    } else {
        crate::vfs_client::mkdir(path.as_bytes())
    };
    match created {
        Ok(()) => {
            let _ = super::refresh::refresh(ctx);
        }
        // A full or read-only volume produced nothing and said nothing.
        Err(code) => super::say::refused(ctx, b"Could not create: ", code),
    }
}
