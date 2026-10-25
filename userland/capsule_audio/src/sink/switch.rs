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

//! Moving the output between a USB device and the fixed sink as one comes
//! ready or goes.

use super::choose::with_usb;
use super::client::{ready_at, Sink};
use crate::mark::mark;

impl Sink {
    /// Asks driver.usb_audio0 whether it has a device and moves to it, or
    /// back to the fixed sink. True when the driver changed: the one left is
    /// told to stop, and the pump starts the new one with its next period.
    pub fn rechoose(&self) -> bool {
        let Some(usb) = self.usb else {
            return false;
        };
        let fixed = self.fixed.map(|(port, _)| port);
        let Some(next) = with_usb(ready_at(usb), fixed, Some(usb)) else {
            return false;
        };
        let was = self.port.replace(next);
        if was == next {
            return false;
        }
        self.stop_at(was, 0);
        mark(self.output_line());
        true
    }
}
