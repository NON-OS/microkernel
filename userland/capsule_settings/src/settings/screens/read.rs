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

//! The displays read from the compositor, through the shell: on entering
//! Appearance, and after each change, so the rows say what the screens do.

use crate::settings::access::shell::{call, OP_DISPLAY_ARRANGEMENT};
use crate::settings::state::status::StatusKind;
use crate::settings::state::State;

use super::wire::shown;

/// The displays as the compositor lays them out now; the chosen display
/// kept while there are that many.
pub fn read(state: &mut State) {
    let mut out = [0u8; 32];
    match call(OP_DISPLAY_ARRANGEMENT, &[], &mut out).ok().and_then(|()| shown(&out)) {
        Some(s) => {
            let output = state.screens.output.min(s.outputs - 1);
            state.screens = super::ScreenPage { shown: s, output, read: true };
        }
        None => {
            state.screens.read = false;
            state.status.set(StatusKind::Error, b"the displays could not be read");
        }
    }
}
