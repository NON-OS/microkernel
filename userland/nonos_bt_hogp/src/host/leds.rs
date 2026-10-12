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
//! The keyboard's lock LEDs. Every boot keyboard output report the
//! peripheral offers with Write Without Response (HIDS 1.0, Boot Keyboard
//! Output Report) gets the lock octet in a Write Command (Vol 3, Part F,
//! 3.4.5.3) whenever the locks change. A command has no response, so the
//! LEDs never wait on, or hold up, a request.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use alloc::vec::Vec;
use nonos_bt_wire::att;

impl Host {
    /// The locks the LEDs show (`report::NUM_LOCK` and the rest).
    pub fn locks(&self) -> u8 {
        self.locks.bits()
    }

    /// The locks as the session holds them, for when they change on
    /// another keyboard. The LEDs are written now if the host is ready,
    /// and when it becomes ready otherwise.
    pub fn set_locks(&mut self, bits: u8) -> Step {
        if !self.locks.set(bits) || self.phase != Phase::Ready {
            return Step::default();
        }
        Step { send: self.leds(), ..Step::default() }
    }

    pub(super) fn leds(&self) -> Vec<Vec<u8>> {
        self.outputs.iter().map(|&h| att::write(h, &[self.locks.bits()], true)).collect()
    }
}
