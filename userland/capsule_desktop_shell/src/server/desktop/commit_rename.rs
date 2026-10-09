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

//! Finish an inline rename: rename the entry on the filesystem and re-sync.

use alloc::string::ToString;

use nonos_app_skeleton::log_line::{say as log, Line};

use super::home::home_path;
use crate::state::{Context, NotifyLevel};

/// A desktop name is one entry: not empty, "." or "..", and no '/'.
const NOT_A_NAME: &[u8] = b"Could not rename: no '/', and not . or ..";

pub fn commit_rename(ctx: &mut Context) {
    super::release_keys::release_keys(ctx);
    let Some(index) = ctx.rename.take() else {
        return;
    };
    let new_name = ctx.rename_buf.trim().to_string();
    ctx.rename_buf.clear();

    let old_name = match ctx.desktop_items.get(index) {
        Some(item) => item.name.clone(),
        None => return,
    };
    if new_name == old_name {
        return;
    }
    // Both names under the home directory the icon was listed from; a name
    // that is not a single entry there ("..", one with a '/') renames nothing,
    // and says so rather than letting the old name snap back unexplained.
    let (Some(old), Some(new)) = (home_path(&old_name), home_path(&new_name)) else {
        ctx.toasts.push(NOT_A_NAME, NotifyLevel::Error, crate::server::toast_clock::now());
        let line = Line::new(b"SHELL").text(b"desktop: rename refused: ");
        let _ = log(&line.text(NOT_A_NAME));
        return;
    };
    match crate::vfs_client::rename(old.as_bytes(), new.as_bytes()) {
        Ok(()) => {
            let _ = super::refresh::refresh(ctx);
        }
        Err(code) => super::say::refused(ctx, b"Could not rename: ", code),
    }
}
