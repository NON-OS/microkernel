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

//! A line that begins with `qwen`: a window asked for, tiers to download or
//! list, help, or a chat on this terminal.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::read_file;
use nonos_libc::mk_time_millis;

use super::ask::parse;
use crate::command::output::Output;
use crate::jobs::{submit, StdinQueue};
use crate::term::cwd::resolve;
use crate::term::state::State;

/// Run `line` when it is a `qwen` line. False, with nothing done, when not.
pub fn enter(state: &mut State, line: &[u8]) -> bool {
    let Some(ask) = parse(line) else { return false };
    if let Some(window) = super::window::parse(&ask) {
        super::open::open(state, ask.question, window);
        return true;
    }
    if let Some(fetch) = super::fetch_words::parse(&ask) {
        super::fetch::enter(state, fetch);
        return true;
    }
    let recorded = ask.recorded();
    state.history.push(&recorded);
    state.last_status = 0;
    if ask.tier.is_none() && matches!(ask.question, b"-h" | b"--help") {
        let _ = crate::command::builtin::help_one::run(
            &mut Output::new(&mut state.scrollback),
            b"qwen",
        );
        return true;
    }
    let tier = ask.tier();
    say_note(state);
    /* A vision tier's ` < FILE` is its image, read here and never kept. */
    let (question, file) = super::image::split(tier, ask.question);
    let image = match file {
        Some(path) => match read_image(state, path, question) {
            Ok(bytes) => bytes,
            Err(line) => {
                state.scrollback.push_line(&line);
                state.last_status = 1;
                return true;
            }
        },
        None => Vec::new(),
    };
    if let Some(work) = super::start::spawn(state, tier, question, &image) {
        let _ = submit(state, &recorded, false, work);
        state.fg_running = true;
        state.fg_started_ms = mk_time_millis();
    }
    true
}

/// The image `path` names, whole, or the line saying why not: what the
/// terminal holds for a program, less the frame and the question.
fn read_image(state: &State, path: &[u8], question: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
    let full = resolve(state.cwd.as_bytes(), path);
    let most = super::image::image_max(StdinQueue::FILE_MAX + 1, question);
    let at = |why: &[u8]| [&b"qwen: "[..], &full[..], b": ", why].concat();
    match read_file(state.owner_pid, &full, most as u32 + 1) {
        Ok(bytes) if bytes.len() > most => Err(at(b"larger than an image a model is handed (1 MiB with the question)")),
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(at(e.as_bytes())),
    }
}

/// The line `chosen` left when it ran another tier than the one chosen.
pub fn say_note(state: &mut State) {
    if let Some(note) = super::chosen::take_note() {
        for line in note.split(|&b| b == b'\n') {
            state.scrollback.push_line(line);
        }
    }
}
