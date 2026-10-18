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

//! A packet onto the kernel input ring, in order: the position or the
//! motion, the wheel, then each button that changed. A post the ring
//! refuses is dropped, as every input driver drops it: the next packet
//! carries the position anew.

use nonos_libc::{
    input_abs_scale, mk_input_event_post, InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP,
    INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL, INPUT_KIND_WHEEL,
};

use crate::constants::RANGE_MAX;
use crate::packet::{Motion, Packet};

#[derive(Default)]
pub struct Pointer {
    /// The last position posted, so a repeat is not posted again.
    pub at: Option<(i32, i32)>,
    pub buttons: u8,
}

impl Pointer {
    pub fn post(&mut self, p: Packet) {
        match p.motion {
            Motion::Abs(x, y) => {
                // The scale holds a host to its grid: past it is its edge.
                let at = (input_abs_scale(x, 0, RANGE_MAX), input_abs_scale(y, 0, RANGE_MAX));
                if self.at != Some(at) && send(INPUT_KIND_POINTER_ABS, 0, at, (0, 0)) {
                    self.at = Some(at);
                }
            }
            Motion::Rel(0, 0) => {}
            Motion::Rel(dx, dy) => {
                send(INPUT_KIND_POINTER_REL, 0, (0, 0), (dx, dy));
                self.at = None;
            }
        }
        if p.wheel != 0 {
            send(INPUT_KIND_WHEEL, 0, (0, 0), (0, p.wheel));
        }
        let changed = self.buttons ^ p.buttons;
        for bit in 0..3u8 {
            if changed & 1 << bit != 0 {
                let down = p.buttons & 1 << bit != 0;
                let kind = if down { INPUT_KIND_BUTTON_DOWN } else { INPUT_KIND_BUTTON_UP };
                send(kind, u32::from(bit) + 1, (0, 0), (0, 0));
            }
        }
        self.buttons = p.buttons;
    }
}

fn send(kind: u16, code: u32, at: (i32, i32), delta: (i32, i32)) -> bool {
    let ev = InputEvent {
        kind,
        flags: 0,
        code,
        x: at.0,
        y: at.1,
        delta_x: delta.0,
        delta_y: delta.1,
        timestamp_ns: 0,
    };
    mk_input_event_post(&ev) >= 0
}
