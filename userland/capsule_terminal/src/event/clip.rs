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

//! Ctrl+Shift+C copies, Ctrl+Shift+V pastes, Ctrl+Shift+F searches. They
//! are taken before a running program sees the key, since without Shift
//! the same letters are the program's own.

use crate::term::state::State;
use nonos_app_skeleton::{clipboard_copy, EventOutcome, InputEvent, MOD_CTRL, MOD_SHIFT};

pub fn clip_key(state: &mut State, event: &InputEvent) -> Option<EventOutcome> {
    let both = MOD_CTRL | MOD_SHIFT;
    if event.flags & both != both {
        return None;
    }
    match char::from_u32(event.code)?.to_ascii_lowercase() {
        'c' if state.sel.is_some() => Some(copy_selection(state)),
        'v' if state.fg_running => Some(super::paste_program::paste_to_program(state)),
        'f' => Some(super::find_bar::open(state)),
        _ => None,
    }
}

fn copy_selection(state: &mut State) -> EventOutcome {
    let Some(sel) = state.sel else { return EventOutcome::Idle };
    let (a, b) = sel.ordered();
    let text = state.scrollback.vt.text_between(a, b, sel.block);
    if clipboard_copy(text.as_bytes()).is_err() {
        state.scrollback.push_line(b"copy: clipboard unavailable");
        return EventOutcome::Repaint;
    }
    EventOutcome::Idle
}
