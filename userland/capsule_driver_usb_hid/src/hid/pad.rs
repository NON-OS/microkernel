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
//! the pad, numbered as pads bind, so two pads are told apart. A pad that
//! goes away lets go of every button it held.

use core::sync::atomic::{AtomicI32, Ordering};

use nonos_libc::{
    mk_input_event_post, InputEvent, INPUT_KIND_PAD_AXIS, INPUT_KIND_PAD_BUTTON_DOWN,
    INPUT_KIND_PAD_BUTTON_UP,
};

use crate::descriptors::{PadEvent, PadLayout, PadState};

static NEXT: AtomicI32 = AtomicI32::new(0);

pub struct Pad {
    id: i32,
    state: PadState,
}

impl Pad {
    pub fn new(layout: PadLayout) -> Self {
        Self { id: NEXT.fetch_add(1, Ordering::Relaxed), state: PadState::new(layout) }
    }

    pub fn feed(&mut self, report: &[u8]) {
        let id = self.id;
        self.state.feed(report, |e| post(id, e));
    }

    pub fn release_all(&mut self) {
        let id = self.id;
        self.state.release(|e| post(id, e));
    }
}

fn post(id: i32, e: PadEvent) {
    let (kind, code, x) = match e {
        PadEvent::Button(n, true) => (INPUT_KIND_PAD_BUTTON_DOWN, n, 0),
        PadEvent::Button(n, false) => (INPUT_KIND_PAD_BUTTON_UP, n, 0),
        PadEvent::Axis(usage, value) => (INPUT_KIND_PAD_AXIS, usage, value),
    };
    let code = code as u32;
    let ev = InputEvent { kind, flags: 0, code, x, y: id, delta_x: 0, delta_y: 0, timestamp_ns: 0 };
    let _ = mk_input_event_post(&ev);
}
