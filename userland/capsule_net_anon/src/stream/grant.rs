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

//! When a stream may be granted more room.

use crate::circuit::window::STREAM_INCREMENT;

use super::table::Stream;

/*
 * A SENDME is the only thing that lets the far end send more, so withholding it
 * while the caller is behind is the whole of the backpressure this transport has.
 * Granting on cell count alone meant an exit could keep sending for as long as it
 * liked into a buffer with no bound: thirty two streams doing that exhaust a
 * sixteen megabyte heap, and nothing about it requires the exit to be hostile,
 * only faster than the reader.
 *
 * Withholding stalls a stream rather than breaking it. The count stays owed, and
 * the grant goes out on a later turn once the caller has drained.
 */

impl Stream {
    /// Whether a SENDME is owed and the caller is keeping up. Resets the count
    /// and reopens the window when it returns true.
    pub fn take_sendme_due(&mut self, high_water: usize) -> bool {
        if self.delivered_since < STREAM_INCREMENT || self.inbound.len() > high_water {
            return false;
        }
        self.delivered_since -= STREAM_INCREMENT;
        self.deliver_window = self.deliver_window.saturating_add(STREAM_INCREMENT);
        true
    }
}
