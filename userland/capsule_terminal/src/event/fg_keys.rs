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

//! Keys while a program runs in the foreground.
//!
//! A program that drew a full screen reads keys raw, as the sequences
//! xterm sends. Any other program reads lines, edited here as a tty's
//! canonical mode edits them. Programs cannot yet say which they want: the
//! terminal has no path for a program's tty settings. Until it does, a
//! program is taken to read raw once it asks for the alternate screen,
//! application cursor keys, bracketed paste or mouse reports, which only
//! raw readers ask for.

use nonos_app_skeleton::{EventOutcome, InputEvent};
use nonos_vt::input::{encode_key, Key, Mods};

use super::fg_input::{reads_raw, send, takes_input};
use super::interrupt::interrupt;
use super::keymap::key_of;
use crate::term::state::State;

pub fn fg_key(state: &mut State, event: InputEvent) -> Option<EventOutcome> {
    if !state.fg_running {
        return None;
    }
    let Some((key, m)) = key_of(&event) else { return Some(EventOutcome::Idle) };
    // Shift with the page keys reads history, as in every terminal.
    if m.shift && !m.ctrl && !state.scrollback.vt.alt_active() {
        let page = state.scrollback.vt.rows().saturating_sub(2).max(1);
        match key {
            Key::PageUp => state.scrollback.scroll_up(page),
            Key::PageDown => state.scrollback.scroll_down(page),
            _ => return Some(raw_or_cooked(state, key, m)),
        }
        return Some(EventOutcome::Repaint);
    }
    Some(raw_or_cooked(state, key, m))
}

fn raw_or_cooked(state: &mut State, key: Key, m: Mods) -> EventOutcome {
    if !takes_input(state) {
        // A built-in job reads nothing; Ctrl+C still stops it.
        if m.ctrl && key == Key::Char('c') {
            return interrupt(state);
        }
        return EventOutcome::Idle;
    }
    state.scrollback.jump_bottom();
    if reads_raw(&state.scrollback.vt) {
        let mut bytes = alloc::vec::Vec::new();
        if encode_key(key, m, &state.scrollback.vt.modes, &mut bytes) {
            send(state, &bytes);
        }
        return EventOutcome::Repaint;
    }
    super::fg_cooked::cooked(state, key, m)
}
