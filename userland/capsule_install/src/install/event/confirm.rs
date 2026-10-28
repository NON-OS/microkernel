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
//! the whole reason the word is there. On a disk an update could keep, Up
//! and Down move between updating and erasing; each move clears the word,
//! so it is typed for the row it starts.

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_UP};

use crate::install::job::{replan, start};
use crate::install::state::{Choice, Screen, State};

pub fn on_confirm_key(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => {
            state.typed.clear();
            (state.prepared, state.keep) = (None, None);
            state.screen = Screen::Disks;
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => {
            state.typed.pop();
            EventOutcome::Repaint
        }
        KEY_UP | KEY_DOWN if state.offers_update() => {
            let to = if code == KEY_UP { Choice::Update } else { Choice::Erase };
            if to != state.choice {
                state.choice = to;
                state.typed.clear();
                replan(state);
            }
            EventOutcome::Repaint
        }
        KEY_ENTER => {
            let word = state.selected_disk().map(|d| d.confirm_word()).unwrap_or_default();
            if state.typed != word.as_bytes() || !matches!(state.prepared, Some(Ok(_))) {
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
