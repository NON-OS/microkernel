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

//! The host's pointer. With the guest taking absolute positions on events
//! and the host's integration on, a move is kept for the guest to read and
//! an event raised; otherwise it goes to the PS/2 mouse as motion.

use super::host::Host;

const GUEST_CAN_ABSOLUTE: u32 = 1;
const NEW_PROTOCOL: u32 = 1 << 4;
const EVENT_MOUSE_POSITION: u32 = 1 << 9;
/// An event this driver never asks for: a display change request.
const EVENT_DISPLAY_CHANGE: u32 = 1 << 2;

impl Host {
    pub fn absolute(&self) -> bool {
        let want = GUEST_CAN_ABSOLUTE | NEW_PROTOCOL;
        self.integration && self.guest_mouse & want == want
    }

    pub fn host_move(&mut self, x: i32, y: i32) {
        if !self.absolute() {
            self.ps2_moves += 1;
            return;
        }
        self.at = (x, y);
        self.pending |= EVENT_MOUSE_POSITION;
        self.raise();
    }

    /// The user resized the host window: an event the filter keeps out.
    pub fn host_resize(&mut self) {
        self.pending |= EVENT_DISPLAY_CHANGE;
        self.raise();
    }

    pub fn pending_flag(&self) -> bool {
        self.memory[2] != 0
    }
}
