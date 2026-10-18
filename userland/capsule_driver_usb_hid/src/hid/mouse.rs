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

use super::mouse_event::MouseEvent;
use super::mouse_report::mouse_event;
use super::post_failures::count_pointer;
use super::post_mouse;

/// One relative pointer interface's state: the buttons its last report
/// held. Its motion is posted as it comes and not kept.
#[derive(Default)]
pub struct Mouse {
    buttons: u8,
}

impl Mouse {
    pub fn new() -> Self {
        Self { buttons: 0 }
    }

    /// Take one boot report from the interrupt endpoint.
    pub fn feed(&mut self, report: &[u8]) {
        if let Some(event) = mouse_event(report, self.buttons) {
            self.apply(event);
        }
    }

    /// Take one report, read in the boot layout or through a report
    /// descriptor: its motion, its wheel and every button that changed.
    pub fn apply(&mut self, event: MouseEvent) {
        count_pointer(post_mouse::publish(event, self.buttons));
        self.buttons = event.buttons;
    }

    /// The mouse went away: every button it held comes up, so a drag ends.
    pub fn release_all(&mut self) {
        self.apply(MouseEvent { dx: 0, dy: 0, dz: 0, buttons: 0 });
    }
}
