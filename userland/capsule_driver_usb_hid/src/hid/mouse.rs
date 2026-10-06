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

use alloc::collections::VecDeque;

use super::mouse_event::MouseEvent;
use super::mouse_report::mouse_event;
use super::post_mouse;

const CAP: usize = 64;

pub struct Mouse {
    buttons: u8,
    events: VecDeque<MouseEvent>,
    post_failures: u64,
}

impl Mouse {
    pub fn new() -> Self {
        Self { buttons: 0, events: VecDeque::new(), post_failures: 0 }
    }

    pub fn feed(&mut self, report: &[u8]) {
        let previous_buttons = self.buttons;
        let Some(event) = mouse_event(report, previous_buttons) else { return };
        self.push(event, previous_buttons);
        self.buttons = event.buttons;
    }

    pub fn pop(&mut self) -> Option<MouseEvent> {
        self.events.pop_front()
    }

    pub fn pending(&self) -> u32 {
        self.events.len() as u32
    }

    pub fn post_failures(&self) -> u64 {
        self.post_failures
    }

    fn push(&mut self, event: MouseEvent, previous_buttons: u8) {
        if !post_mouse::publish(event, previous_buttons) {
            self.post_failures = self.post_failures.wrapping_add(1);
        }
        if self.events.len() < CAP {
            self.events.push_back(event);
        }
    }
}
