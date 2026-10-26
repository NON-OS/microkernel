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
use crate::hid::{MouseLayout, Multitouch, TouchLayout};
use crate::input::TouchGesture;

impl State {
    pub fn new() -> Self {
        Self {
            descriptor: [0; 30],
            descriptor_len: 0,
            input_register: 0,
            input_len: 0,
            last_buttons: 0,
            input_polls: 0,
            input_reports: 0,
            post_failures: 0,
            touch_decoded: false,
            polls_since_touch: 0,
            doorbell_proven: false,
            repeat: FrameRepeat::default(),
            touch_layout: TouchLayout::default(),
            mt: Multitouch::default(),
            mouse_layout: MouseLayout::default(),
            gesture: TouchGesture::default(),
            said: false,
        }
    }

    pub fn found(&self) -> bool {
        self.descriptor_len != 0
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
