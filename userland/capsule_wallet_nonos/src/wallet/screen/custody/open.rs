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
//! Opening a key screen, and showing the key on the export screen.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::{State, VIEW_EXPORT, VIEW_IMPORT, VIEW_RECOVER};

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
