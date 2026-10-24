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

//! Post a touchscreen's events (multitouch/screen.rs): a press as
//! INPUT_KIND_TOUCH where the finger lands, a drag as POINTER_ABS, a lift
//! as button 1 up, positions in the router's 0 to 7FFFh range.

use nonos_libc::{
    mk_input_event_post, InputEvent, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_TOUCH,
};

use crate::hid::ScreenEvent;
use crate::state::State;

pub fn publish_screen(state: &mut State, events: &[Option<ScreenEvent>; 2]) {
    for e in events.iter().flatten() {
        let (kind, code, x, y) = match *e {
            ScreenEvent::Press(x, y) => (INPUT_KIND_TOUCH, 1, x, y),
            ScreenEvent::Move(x, y) => (INPUT_KIND_POINTER_ABS, 0, x, y),
            ScreenEvent::Release => (INPUT_KIND_BUTTON_UP, 1, 0, 0),
        };
        let (x, y) = (x as i32, y as i32);
        let ev = InputEvent { kind, flags: 0, code, x, y, delta_x: 0, delta_y: 0, timestamp_ns: 0 };
        if mk_input_event_post(&ev) < 0 {
            state.post_failures = state.post_failures.wrapping_add(1);
        }
    }
}
