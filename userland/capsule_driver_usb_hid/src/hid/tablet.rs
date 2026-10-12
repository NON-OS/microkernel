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

use nonos_libc::{INPUT_KIND_POINTER_ABS, INPUT_KIND_WHEEL};

use super::post_failures::count_pointer;
use super::post_mouse::publish_buttons;
use super::post_wire::{send, send_abs};

/// What one absolute pointer report says, its position scaled onto the
/// 0 to 0x7FFF range the input router maps onto the screen. `at` is false
/// when the report has no position to go by: a pen out of range, a finger
/// lifted off a touchscreen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabletReport {
    pub buttons: u8,
    pub x: i32,
    pub y: i32,
    pub wheel: i32,
    pub at: bool,
}

/// One absolute pointer interface's state: the buttons its last report
/// held.
#[derive(Default)]
pub struct Tablet {
    buttons: u8,
}

impl Tablet {
    pub fn new() -> Self {
        Self { buttons: 0 }
    }

    pub fn apply(&mut self, report: TabletReport) {
        let mut ok = true;
        if report.at {
            ok &= send_abs(INPUT_KIND_POINTER_ABS, report.x, report.y);
        }
        if report.wheel != 0 {
            ok &= send(INPUT_KIND_WHEEL, 0, 0, 0, report.wheel);
        }
        count_pointer(ok & publish_buttons(self.buttons, report.buttons));
        self.buttons = report.buttons;
    }

    /// The pointer went away: every button it held comes up.
    pub fn release_all(&mut self) {
        self.apply(TabletReport { buttons: 0, x: 0, y: 0, wheel: 0, at: false });
    }
}
