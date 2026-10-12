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

//! One key into the recovery-phrase field.

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use super::recover::wipe;
use super::recover_submit::submit;
use crate::wallet::state::State;

pub fn recover_input(state: &mut State, code: u32) -> EventOutcome {
    if code == KEY_ESC {
        wipe(state);
        state.recover_active = false;
        state.status = b"recovery cancelled";
        return EventOutcome::Repaint;
    }
    if code == KEY_ENTER {
        return submit(state);
    }
    if code == KEY_BACKSPACE {
        if state.recover_len > 0 {
            state.recover_len -= 1;
            // SAFETY: volatile write so removed characters do not linger.
            unsafe { core::ptr::write_volatile(&mut state.recover_buf[state.recover_len], 0) };
        }
        return EventOutcome::Repaint;
    }
    let ch = code as u8;
    let lower = ch.to_ascii_lowercase();
    let is_letter = lower.is_ascii_lowercase();
    let is_space = ch == b' ';
    if (is_letter || is_space) && state.recover_len < state.recover_buf.len() {
        // Collapse doubled separators so word splitting stays unambiguous.
        if is_space && (state.recover_len == 0 || state.recover_buf[state.recover_len - 1] == b' ')
        {
            return EventOutcome::Idle;
        }
        state.recover_buf[state.recover_len] = if is_space { b' ' } else { lower };
        state.recover_len += 1;
    }
    EventOutcome::Repaint
}
