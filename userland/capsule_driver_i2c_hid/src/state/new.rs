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

//! A fresh state for a pad not yet found, and whether it has been.

use super::{FrameRepeat, State};
use crate::hid::{MouseLayout, TouchLayout};
use crate::input::TouchGesture;

impl State {
    pub fn new(i2c_port: u32, i2c_pid: u32) -> Self {
        Self {
            i2c_port,
            i2c_pid,
            addr: 0,
            descriptor: [0; 30],
            descriptor_len: 0,
            input_register: 0,
            input_len: 0,
            last_buttons: 0,
            probes: 0,
            input_polls: 0,
            input_reports: 0,
            post_failures: 0,
            woke: false,
            touch_decoded: false,
            polls_since_touch: 0,
            doorbell_proven: false,
            repeat: FrameRepeat::default(),
            frame_dumps: 0,
            touch_layout: TouchLayout::default(),
            mouse_layout: MouseLayout::default(),
            gesture: TouchGesture::default(),
            doorbell_absent: false,
            doorbell_failures: 0,
            doorbell_misses: 0,
            last_read_had_report: false,
            said: false,
        }
    }

    pub fn found(&self) -> bool {
        self.descriptor_len != 0
    }
}
