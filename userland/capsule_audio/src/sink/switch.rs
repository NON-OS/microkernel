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

//! Moving the output between the USB device and the machine's own sink as
//! one comes ready or goes.

use nonos_audio_proto::OUTPUT_READY;

use super::choose::{choose, SINK_LINES};
use super::client::{status_at, Sink};
use crate::mark::mark;

impl Sink {
    /// Asks driver.usb_audio0 whether it has a device and moves to it, or
    /// back. True when the driver changed: the one left is told to stop, and
    /// the pump starts the new one with its next period.
    pub fn rechoose(&self) -> bool {
        let Some(usb) = self.usb else {
            return false;
        };
        let ready = matches!(status_at(usb, 0), Some((OUTPUT_READY, _)));
        let Some(next) = choose(ready, self.own.map(|(port, _)| port), Some(usb)) else {
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

    /// The start's and each move's log line.
    pub fn output_line(&self) -> &'static str {
        match self.own {
            Some((port, i)) if port == self.port.get() => SINK_LINES[i],
            _ => "[AUDIO] output driver.usb_audio0\n",
        }
    }
}
