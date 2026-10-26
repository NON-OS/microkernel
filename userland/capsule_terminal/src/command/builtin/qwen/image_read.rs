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

//! A vision tier's ` < FILE`: the image read for it, whole, and never kept.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::read_file;

use crate::jobs::StdinQueue;
use crate::term::cwd::resolve;
use crate::term::state::State;

/// The question to ask `tier` and the image to hand it (empty: none), or
/// None once the line saying why the image cannot be read is shown.
pub fn take<'a>(state: &mut State, tier: &[u8], line: &'a [u8]) -> Option<(&'a [u8], Vec<u8>)> {
    let (question, file) = super::image::split(tier, line);
    let Some(path) = file else { return Some((question, Vec::new())) };
    match read_image(state, path, question) {
        Ok(bytes) => Some((question, bytes)),
        Err(line) => {
            state.scrollback.push_line(&line);
            state.last_status = 1;
            None
        }
    }
}

/// The image `path` names, whole, or the line saying why not: what the
/// terminal holds for a program, less the frame and the question.
fn read_image(state: &State, path: &[u8], question: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
    let full = resolve(state.cwd.as_bytes(), path);
    let most = super::image::image_max(StdinQueue::FILE_MAX + 1, question);
    let at = |why: &[u8]| [&b"qwen: "[..], &full[..], b": ", why].concat();
    match read_file(state.owner_pid, &full, most as u32 + 1) {
        Ok(bytes) if bytes.len() > most => Err(at(TOO_LARGE)),
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(at(e.as_bytes())),
    }
}

const TOO_LARGE: &[u8] = b"larger than an image a model is handed (1 MiB with the question)";
