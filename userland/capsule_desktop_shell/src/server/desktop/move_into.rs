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

//! Move a desktop item into a folder by renaming it under that folder's path,
//! then re-sync. Both indices are into the root listing.

use super::home::home_path;
use crate::state::Context;
use alloc::format;

pub fn move_into(ctx: &mut Context, src: usize, folder: usize) {
    if src == folder {
        return;
    }
    let src_name = match ctx.desktop_items.get(src) {
        Some(item) => item.name.clone(),
        None => return,
    };
    let folder_item = match ctx.desktop_items.get(folder) {
        Some(item) => item,
        None => return,
    };
    if !folder_item.is_dir {
        return;
    }
    // Same home prefix the listing uses, for the same reason.
    let (Some(old), Some(folder_path)) = (home_path(&src_name), home_path(&folder_item.name))
    else {
        return;
    };
    let new = format!("{folder_path}/{src_name}");
    match crate::vfs_client::rename(old.as_bytes(), new.as_bytes()) {
        Ok(()) => {
            let _ = super::refresh::refresh(ctx);
        }
        // The icon just sprang back to where it started, unexplained.
        Err(code) => super::say::refused(ctx, b"Could not move: ", code),
    }
}
