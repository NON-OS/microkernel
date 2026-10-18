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

//! Which root ports speak USB 2 and which USB 3, from the Supported Protocol
//! capabilities (xHCI 1.2 section 7.2). Real controllers expose the two as
//! separate port ranges, one physical connector holding a port in each: an
//! Intel PCH may number its USB 2 ports 1 to 12 and its USB 3 ports 13 to
//! 18. The two kinds are reset differently and their EP0 starts at a
//! different packet size, so each port's protocol is read before it is
//! touched.

const PORT_SLOTS: usize = 256;

/// The USB major revision of each root port, 0 where no capability names it.
#[derive(Clone, Copy)]
pub struct PortProtocols {
    major: [u8; PORT_SLOTS],
}

impl PortProtocols {
    pub const fn unknown() -> Self {
        Self { major: [0; PORT_SLOTS] }
    }

    /// Ports `first` to `first + count - 1` (1-based) speak USB `major`.
    /// Ports past 255 do not exist and are ignored.
    pub fn record(&mut self, major: u8, first: u8, count: u8) {
        if first == 0 {
            return;
        }
        let start = first as usize;
        let end = (start + count as usize).min(PORT_SLOTS);
        for port in start..end {
            self.major[port] = major;
        }
    }

    /// The USB major revision of `port`, 0 when unknown.
    pub fn major(&self, port: u8) -> u8 {
        self.major[port as usize]
    }

    /// A USB 3 (SuperSpeed or faster) port.
    pub fn is_usb3(&self, port: u8) -> bool {
        self.major(port) >= 3
    }
}
