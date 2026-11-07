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
    /// The guest acked `serial`. Only the configure last sent is taken, under
    /// any serial it was told under: a client may ack a configure before it
    /// reads the one that told the same mode and size again, as a focus
    /// change does (again), and an ack of the first was dropped, so the
    /// buffer drawn for a maximise was shown at the old corner, in the old
    /// frame. An ack of a configure it replaced changes nothing. Serials
    /// count up and wrap, so the span is measured from the first.
    pub fn ack(&mut self, serial: u32) {
        let from = self.sent_from;
        if self.sent.is_some_and(|c| serial.wrapping_sub(from) <= c.serial.wrapping_sub(from)) {
            self.acked = self.sent.take();
            self.acked_held = self.sent_held.take();
        }
    }

    /// The acked configure the next buffer is drawn for.
    pub fn due(&self) -> Option<Configure> {
        self.acked
    }

    /// That buffer is on screen: the window is in its new mode.
    pub fn shown(&mut self) {
        self.acked_held = None;
        if let Some(c) = self.acked.take() {
            self.mode = c.mode;
            if c.mode == Mode::Normal {
                self.saved = None;
            }
        }
    }
}
