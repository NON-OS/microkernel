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

//! A report on the multitouch path. A touchscreen's frames are posted here
//! as absolute pointer events; a parallel-mode pad's frame goes on as the
//! one sample the gesture engine reads, its first finger down and how many
//! are, like any touch report. A report of another ID (a pad's mouse
//! collection, a pointing stick) is read as pointer input as before.

use super::pointer::pointer;
use crate::hid::{primary, Surface};
use crate::input::publish_screen::publish_screen;
use crate::state::{Seen, State};

/// What the report says for the repeat check and the gesture engine; None
/// when it completes no frame (a torn read, a hybrid frame still coming),
/// or completed a screen's, which is posted already.
pub(super) fn multitouch(state: &mut State, report: &[u8]) -> Option<Seen> {
    if !state.mt.takes(report) {
        return pointer(state, report);
    }
    let f = state.mt.frame(report)?;
    let max = state.mt.max();
    if state.mt.surface == Surface::Screen {
        let events = state.mt.screen.step(&f, max);
        publish_screen(state, &events);
        return None;
    }
    Some(Seen::Touch(primary(&f, max)))
}
