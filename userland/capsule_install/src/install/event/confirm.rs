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

//! The confirmation: type the disk's word, then Enter. A single key cannot
//! start an erase, and Enter with the wrong word does nothing, which is
//! the whole reason the word is there.

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use crate::install::job::start;
use crate::install::state::{Screen, State};

pub fn on_confirm_key(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => {
            state.typed.clear();
            state.screen = Screen::Disks;
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => {
            state.typed.pop();
            EventOutcome::Repaint
        }
        KEY_ENTER => {
            let word = state.selected_disk().map(|d| d.confirm_word()).unwrap_or_default();
            if state.typed != word.as_bytes() {
                return EventOutcome::Idle;
            }
            match start(state) {
                Ok(()) => state.screen = Screen::Writing,
                Err(why) => {
                    state.notice = Some(why);
                    state.screen = Screen::Failed;
                }
            }
            EventOutcome::Repaint
        }
        c if (0x20..0x7F).contains(&c) && state.typed.len() < 16 => {
            state.typed.push((c as u8).to_ascii_lowercase());
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}
