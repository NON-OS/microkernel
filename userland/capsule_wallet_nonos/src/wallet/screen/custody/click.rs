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
//! Opening, typing into and closing the key screens.

use nonos_app_skeleton::{EventOutcome, KEY_ENTER, KEY_ESC};

use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_EXPORT, VIEW_HOME, VIEW_IMPORT, VIEW_RECOVER, VIEW_SETTINGS};

/// Where a key screen goes back to: Settings once there is an account,
/// the first screen before.
fn leave(state: &mut State) {
    state.view = if state.address_ready { VIEW_SETTINGS } else { VIEW_HOME };
    state.scroll = 0;
}

pub fn open(state: &mut State, view: u8) -> EventOutcome {
    state.failure = None;
    state.custody_armed = false;
    state.recover_shown = false;
    state.scroll = 0;
    state.view = view;
    match view {
        VIEW_IMPORT if !state.import_active => {
            let _ = crate::wallet::event::toggle_import(state);
        }
        VIEW_RECOVER if !state.recover_active => {
            let _ = crate::wallet::event::toggle_recover(state);
        }
        _ => {}
    }
    state.view = view;
    EventOutcome::Repaint
}

/// Ask the keyring for the key and show it. Reached only from the export
/// screen's own button, after its warning: Settings opens that screen with
/// the key still hidden.
pub fn reveal(state: &mut State) -> EventOutcome {
    if !state.export_active {
        let _ = crate::wallet::event::toggle_export(state);
    }
    if state.export_active {
        state.failure = None;
        state.view = VIEW_EXPORT;
    } else {
        state.failure = Some("The keyring would not show the key.");
    }
    state.scroll = 0;
    EventOutcome::Repaint
}

/// Close whichever key screen is up, wiping what it held.
pub fn cancel(state: &mut State) -> EventOutcome {
    state.custody_armed = false;
    state.recover_shown = false;
    if state.import_active {
        let _ = crate::wallet::event::toggle_import(state);
    }
    if state.recover_active {
        let _ = crate::wallet::event::toggle_recover(state);
    }
    if state.export_active {
        let _ = crate::wallet::event::toggle_export(state);
    }
    state.failure = None;
    leave(state);
    EventOutcome::Repaint
}

fn submit(state: &mut State) -> EventOutcome {
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

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Dismiss => {
            state.failure = None;
            EventOutcome::Repaint
        }
        Press::Back | Press::Footer(1) => cancel(state),
        Press::Footer(0) if state.view == VIEW_EXPORT && state.export_active => cancel(state),
        Press::Footer(0) if state.view == VIEW_EXPORT => reveal(state),
        Press::Footer(0) => submit(state),
        Press::Footer(2) if state.view == VIEW_RECOVER => {
            state.recover_shown = !state.recover_shown;
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
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
