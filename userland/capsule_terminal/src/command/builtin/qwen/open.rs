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

//! Asking the kernel for `qwen window [tier]` and saying at once what it
//! answered. The window is not this tab's: nothing is attached to the
//! screen and no job waits, so the prompt is back as soon as this returns.

use nonos_libc::mk_tool_run;

use super::window::{request, Window, WORD};
use crate::term::state::State;

/// A line that began `qwen window`, with `typed` the line after `qwen`.
/// History keeps `typed` for a window that was asked for; after a word
/// that names no tier it keeps only `qwen window`, since the rest may have
/// been the start of a question.
pub fn open(state: &mut State, typed: &[u8], window: Window<'_>) {
    let kept = if matches!(window, Window::Open(_)) { typed } else { WORD };
    state.history.push(&[&b"qwen "[..], kept].concat());
    ask(state, window);
}

/// Ask the kernel for the window `window` names and say at once what it
/// answered, on one line.
pub fn ask(state: &mut State, window: Window<'_>) {
    super::enter::say_note(state);
    let Window::Open(tier) = window else {
        state
            .scrollback
            .push_error(b"qwen window: that is not a tier; `help qwen` lists them (EINVAL)");
        state.last_status = 1;
        return;
    };
    let said = [WORD, b" ", tier].concat();
    let rc = mk_tool_run(b"tool.qwen", &request(tier));
    if rc < 0 {
        super::refused::refused(state, &said, rc);
        return;
    }
    let line =
        [b"qwen ", &said[..], b": the window is opening; the chat runs there, not in this tab"];
    state.scrollback.push_line(&line.concat());
    state.last_status = 0;
}
