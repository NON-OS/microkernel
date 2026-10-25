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

use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_EXPORT, VIEW_RECOVER};

pub use super::cancel::cancel;
pub use super::open::{open, reveal};
pub use super::submit::key;
use super::submit::submit;

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    match press {
        Press::Dismiss => {
            state.failure = None;
            EventOutcome::Repaint
        }
        Press::Back | Press::Footer(1) => cancel(state),
        Press::Footer(0) if state.view == VIEW_EXPORT && state.keys.export_active => cancel(state),
        Press::Footer(0) if state.view == VIEW_EXPORT => reveal(state),
        Press::Footer(0) => submit(state),
        Press::Footer(2) if state.view == VIEW_RECOVER => {
            state.keys.recover_shown = !state.keys.recover_shown;
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}
