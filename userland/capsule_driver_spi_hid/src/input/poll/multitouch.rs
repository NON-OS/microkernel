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

//! A frame on the multitouch path: a touchscreen's posted as absolute
//! pointer events, a parallel-mode pad's handed to the gesture engine as
//! its first finger down and how many are. A report that completes no
//! frame (another id, a torn read, a hybrid frame still coming) is used up
//! here all the same: this device's reports are all touch reports.

use crate::hid::{primary, Surface};
use crate::input::publish_screen::publish_screen;
use crate::input::publish_touch::publish_touch;
use crate::state::State;

pub(super) fn multitouch_frame(state: &mut State, body: &[u8]) -> bool {
    let Some(f) = state.mt.frame(body) else { return true };
    (state.touch_decoded, state.polls_since_touch) = (true, 0);
    let max = state.mt.max();
    if state.mt.surface == Surface::Screen {
        let events = state.mt.screen.step(&f, max);
        publish_screen(state, &events);
    } else {
        let act = state.gesture.on_touch(&primary(&f, max));
        publish_touch(state, &act);
    }
    true
}
