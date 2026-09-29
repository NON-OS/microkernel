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

//! Loading a file into a document.
//!
//! The path becomes the document's only once its bytes are in the buffer. A
//! failed read leaves the document exactly as it was, name included, so the
//! next save cannot write the old text over the file that failed to open.

use nonos_app_skeleton::clients::vfs;

use super::mode::mode_for_path;
use super::resolve_owner_pid::resolve_owner_pid;
use super::state::{State, CAPACITY};

/// Read `path` into `state`. On failure `status` says why and nothing else in
/// the document changes.
pub(super) fn load(state: &mut State, path: &[u8]) -> bool {
    if path.is_empty() || path.len() > state.path.len() {
        state.status = b"no path given";
        return false;
    }
    if !resolve_owner_pid(state) {
        state.status = b"open failed: file service not reachable";
        return false;
    }
    if let Ok((size, _)) = vfs::stat(state.owner_pid, path) {
        if size > CAPACITY as u64 {
            state.status = b"open refused: file is larger than 256 KiB";
            return false;
        }
    }
    let bytes = match vfs::read_file(state.owner_pid, path, CAPACITY as u32) {
        Ok(b) => b,
        Err(_) => {
            state.status = b"open failed: file could not be read";
            return false;
        }
    };
    if bytes.len() > CAPACITY {
        state.status = b"open refused: file is larger than 256 KiB";
        return false;
    }
    if core::str::from_utf8(&bytes).is_err() {
        state.status = b"open refused: file is not valid UTF-8";
        return false;
    }
    state.buf[..bytes.len()].copy_from_slice(&bytes);
    state.len = bytes.len();
    state.path[..path.len()].copy_from_slice(path);
    state.path_len = path.len();
    state.reset_history();
    state.status = b"opened";
    state.caret = 0;
    state.scroll_line = 0;
    let p = core::str::from_utf8(path).unwrap_or("");
    state.mode = mode_for_path(p);
    state.reflow();
    true
}
