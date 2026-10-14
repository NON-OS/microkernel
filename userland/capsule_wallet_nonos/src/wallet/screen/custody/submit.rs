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
//! Submitting a key screen and typing into its field.

use nonos_app_skeleton::{EventOutcome, KEY_ENTER, KEY_ESC};

use crate::wallet::state::{State, VIEW_EXPORT, VIEW_HOME, VIEW_IMPORT, VIEW_RECOVER};

use super::cancel::cancel;

pub(super) fn submit(state: &mut State) -> EventOutcome {
    if let Err(why) = crate::wallet::event::may_replace(state) {
        state.failure = Some(why);
        return EventOutcome::Repaint;
    }
    if state.view == VIEW_IMPORT {
        let _ = crate::wallet::event::import_input(state, KEY_ENTER);
        if state.import_active {
            state.failure = core::str::from_utf8(state.status).ok();
            return EventOutcome::Repaint;
        }
    } else {
        let _ = crate::wallet::event::recover_input(state, KEY_ENTER);
        if state.recover_active {
            state.failure = core::str::from_utf8(state.status).ok();
            return EventOutcome::Repaint;
        }
    }
    state.failure = None;
    state.view = VIEW_HOME;
    state.scroll = 0;
    EventOutcome::Repaint
}
/// Every key while a key screen is up goes to its field.
pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    match state.view {
        VIEW_IMPORT | VIEW_RECOVER | VIEW_EXPORT if code == KEY_ESC => Some(cancel(state)),
        VIEW_IMPORT | VIEW_RECOVER if code == KEY_ENTER => Some(submit(state)),
        VIEW_IMPORT => {
            state.failure = None;
            state.custody_armed = false;
            Some(crate::wallet::event::import_input(state, code))
        }
        VIEW_RECOVER => {
            state.failure = None;
            state.custody_armed = false;
            Some(crate::wallet::event::recover_input(state, code))
        }
        VIEW_EXPORT => Some(EventOutcome::Idle),
        _ => None,
    }
}
