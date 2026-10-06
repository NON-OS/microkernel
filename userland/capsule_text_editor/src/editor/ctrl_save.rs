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

//! Writing a document to a file.
//!
//! The document takes the new name only when the write lands, so a Save As
//! that fails leaves the document named after the file that holds its text.

use nonos_app_skeleton::{clients::vfs, EventOutcome};

use super::path_prompt;
use super::resolve_owner_pid::resolve_owner_pid;
use super::save_said::save_failed;
use super::state::{PromptOp, State};

pub(super) fn ctrl_save(state: &mut State) -> EventOutcome {
    // An untitled document has nowhere to go yet, so Save means Save As.
    if state.path_len == 0 {
        return path_prompt::start(state, PromptOp::Save);
    }
    let path = state.path[..state.path_len].to_vec();
    write_to(state, &path);
    EventOutcome::Repaint
}

/// Whether a file is already at `path`, for asking before replacing it.
pub(super) fn exists(state: &mut State, path: &[u8]) -> bool {
    resolve_owner_pid(state) && vfs::stat(state.owner_pid, path).is_ok()
}

pub(super) fn write_to(state: &mut State, path: &[u8]) -> bool {
    if path.is_empty() || path.len() > state.path.len() {
        state.status = b"save failed: no path given";
        return false;
    }
    if !resolve_owner_pid(state) {
        state.status = b"save failed: file service not reachable";
        return false;
    }
    if let Err(err) = vfs::write_file(state.owner_pid, path, &state.buf[..state.len]) {
        state.status = save_failed(err);
        return false;
    }
    state.path[..path.len()].copy_from_slice(path);
    state.path_len = path.len();
    state.mark_saved();
    super::notify::notify_saved(state);
    // The file takes the text; the ribbon's formatting is not in it.
    state.status = if state.has_formatting() {
        b"saved the text; formatting goes out with Export (.docx, .pdf, .md)"
    } else {
        b"saved"
    };
    true
}
