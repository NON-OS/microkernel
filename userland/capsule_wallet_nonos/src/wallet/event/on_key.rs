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

//! A key, given first to whatever owns the keyboard: the backup words, a
//! key screen, a field on the screen on show. Only what none of them takes
//! moves between screens, and no single key ever signs or sends anything.

use nonos_app_skeleton::{EventOutcome, KEY_ENTER, KEY_ESC};

use crate::wallet::screen;
use crate::wallet::state::{State, VIEW_SWAP};

pub fn on_key(state: &mut State, code: u32) -> EventOutcome {
    if state.locked {
        if code == KEY_ENTER {
            return super::lock::unlock(state);
        }
        return EventOutcome::Idle;
    }
    // The one-time backup screen owns input until the user confirms the
    // phrase is written down. Enter is the only way through.
    if state.backup_active {
        if code == KEY_ENTER {
            return super::backup::confirm_backup(state);
        }
        return EventOutcome::Idle;
    }
    if let Some(out) = screen::custody::click::key(state, code) {
        return out;
    }
    if let Some(out) = super::etna_scroll::scroll_key(state, code) {
        return out;
    }
    if let Some(out) = screen::shield::key::key(state, code) {
        return out;
    }
    if let Some(out) = screen::pay::key::key(state, code) {
        return out;
    }
    if let Some(out) = screen::stake::click::key(state, code) {
        return out;
    }
    if state.view == VIEW_SWAP {
        if let Some(out) = super::swap_input::swap_input(state, code) {
            return out;
        }
    }
    if code == KEY_ESC {
        return super::escape::escape(state);
    }
    if code == u32::from(b'r') || code == u32::from(b'R') {
        return super::probe_tick::probe_kick(state);
    }
    EventOutcome::Idle
}
