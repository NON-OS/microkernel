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

/*
 * The readline keys a shell user types without thinking, beyond the ones
 * on_ctrl owns: Ctrl-F and Ctrl-H step and rub out a character, Ctrl-P and
 * Ctrl-N walk history, and Alt with B, F and D moves and cuts by word.
 * Ctrl-B is the rail's, taken before the line sees it.
 */

use nonos_app_skeleton::{EventOutcome, MOD_ALT, MOD_ALTGR, MOD_CTRL};

use super::bool_to_outcome::bool_to_outcome;
use crate::term::state::State;

pub fn readline_key(state: &mut State, code: u32, flags: u16) -> Option<EventOutcome> {
    /* AltGr is how some layouts type characters: a key with it is text. */
    if flags & MOD_ALTGR != 0 {
        return None;
    }
    let letter = char::from_u32(code)?.to_ascii_lowercase();
    if flags & MOD_CTRL != 0 {
        return match letter {
            'f' => Some(bool_to_outcome(state.line.move_right())),
            'h' => Some(bool_to_outcome(state.line.backspace())),
            'p' => Some(super::on_up::on_up(state)),
            'n' => Some(super::on_down::on_down(state)),
            _ => unbound(code),
        };
    }
    if flags & MOD_ALT != 0 {
        return match letter {
            'b' => Some(bool_to_outcome(state.line.move_word_left())),
            'f' => Some(bool_to_outcome(state.line.move_word_right())),
            'd' => Some(bool_to_outcome(state.line.delete_word_right())),
            _ => unbound(code),
        };
    }
    None
}

/*
 * A chord no binding took: typing its letter would put text on the line
 * that nobody typed, so a printable one is swallowed. Named keys go on to
 * the line editor, which gives Ctrl-Home and the like their plain meaning.
 */
fn unbound(code: u32) -> Option<EventOutcome> {
    (0x20..=0x7E).contains(&code).then_some(EventOutcome::Idle)
}
