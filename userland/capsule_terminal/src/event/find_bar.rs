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

//! The scrollback search bar. Typing searches back from the newest line,
//! Enter steps to the next older match and Shift+Enter to the newer one,
//! Alt+C toggles case, Esc closes. The view follows the match.

use nonos_app_skeleton::{
    EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, KEY_ESC, MOD_ALT, MOD_SHIFT,
};

use super::find_seek::{end, reveal, seek};
use crate::term::select::Find;
use crate::term::state::State;

const MAX_QUERY: usize = 256;

pub fn open(state: &mut State) -> EventOutcome {
    state.find = Some(Find::default());
    EventOutcome::Repaint
}

pub fn key(state: &mut State, event: InputEvent) -> EventOutcome {
    let Some(mut f) = state.find.take() else { return EventOutcome::Idle };
    let alt = event.flags & MOD_ALT != 0;
    match event.code {
        KEY_ESC => {
            state.scrollback.jump_bottom();
            return EventOutcome::Repaint;
        }
        KEY_ENTER => {
            let from = f.hit.map(|h| h.0).unwrap_or(end(state));
            f.hit = seek(state, &f, from, event.flags & MOD_SHIFT == 0).or(f.hit);
        }
        KEY_BACKSPACE => {
            f.query.pop();
            f.hit = seek(state, &f, end(state), true);
        }
        c if alt && matches!(c, 0x43 | 0x63) => {
            f.case = !f.case;
            f.hit = seek(state, &f, end(state), true);
        }
        c => {
            let Some(ch) = char::from_u32(c).filter(|ch| !ch.is_control()) else {
                state.find = Some(f);
                return EventOutcome::Idle;
            };
            if f.query.len() < MAX_QUERY {
                f.query.push(ch);
            }
            f.hit = seek(state, &f, end(state), true);
        }
    }
    if let Some((a, _)) = f.hit {
        reveal(state, a.line);
    }
    state.find = Some(f);
    EventOutcome::Repaint
}
