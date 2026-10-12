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
//! A gamepad bound: its reports read against the layout its descriptor
//! gave, each change posted to the input router as a pad event. `y` names
//! the pad, numbered as pads bind, so two pads are told apart.

use core::sync::atomic::{AtomicI32, Ordering};

use nonos_libc::{
    mk_input_event_post, InputEvent, INPUT_KIND_PAD_AXIS, INPUT_KIND_PAD_BUTTON_DOWN,
    INPUT_KIND_PAD_BUTTON_UP,
};

use crate::descriptors::{PadEvent, PadLayout, PadState};

static NEXT: AtomicI32 = AtomicI32::new(0);

pub fn new_pad(layout: PadLayout) -> (i32, PadState) {
    (NEXT.fetch_add(1, Ordering::Relaxed), PadState::new(layout))
}

pub fn feed_pad(id: i32, state: &mut PadState, report: &[u8]) {
    state.feed(report, |e| {
        let (kind, code, x) = match e {
            PadEvent::Button(n, true) => (INPUT_KIND_PAD_BUTTON_DOWN, n, 0),
            PadEvent::Button(n, false) => (INPUT_KIND_PAD_BUTTON_UP, n, 0),
            PadEvent::Axis(usage, value) => (INPUT_KIND_PAD_AXIS, usage, value),
        };
        let code = code as u32;
        let ev =
            InputEvent { kind, flags: 0, code, x, y: id, delta_x: 0, delta_y: 0, timestamp_ns: 0 };
        let _ = mk_input_event_post(&ev);
    });
}
