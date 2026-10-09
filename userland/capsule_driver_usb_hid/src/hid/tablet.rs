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

use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_WHEEL,
};

use super::button_changes::button_changes;
use super::post_wire::{send, send_abs};
use super::tablet_report::{tablet_report, TABLET_BUTTONS};

pub struct Tablet {
    buttons: u8,
}

impl Tablet {
    pub fn new() -> Self {
        Self { buttons: 0 }
    }

    pub fn feed(&mut self, raw: &[u8]) {
        let Some(report) = tablet_report(raw) else { return };
        let _ = send_abs(INPUT_KIND_POINTER_ABS, report.x, report.y);
        if report.wheel != 0 {
            let _ = send(INPUT_KIND_WHEEL, 0, 0, 0, report.wheel);
        }
        publish_buttons(self.buttons, report.buttons);
        self.buttons = report.buttons;
    }
}

fn publish_buttons(previous: u8, current: u8) {
    button_changes(previous, current, TABLET_BUTTONS, |button, down| {
        let kind = if down { INPUT_KIND_BUTTON_DOWN } else { INPUT_KIND_BUTTON_UP };
        let _ = send(kind, 0, button, 0, 0);
    });
}
