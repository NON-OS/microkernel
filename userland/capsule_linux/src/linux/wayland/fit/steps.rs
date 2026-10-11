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

//! A configure acked, its buffer due, and the window shown in its mode.
//! Pure, so the host proofs hold it.

use super::mode::{Configure, Mode};
use super::state::Fit;

impl Fit {
    /// The guest acked `serial`. Only the configure last sent is taken; an
    /// ack of one it replaced changes nothing.
    pub fn ack(&mut self, serial: u32) {
        if self.sent.is_some_and(|c| c.serial == serial) {
            self.acked = self.sent.take();
        }
    }

    /// The acked configure the next buffer is drawn for.
    pub fn due(&self) -> Option<Configure> {
        self.acked
    }

    /// That buffer is on screen: the window is in its new mode.
    pub fn shown(&mut self) {
        if let Some(c) = self.acked.take() {
            self.mode = c.mode;
            if c.mode == Mode::Normal {
                self.saved = None;
            }
        }
    }
}
