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

use super::xecp::walk_ext_caps;
use crate::constants::HCCPARAMS1;
use crate::regs::mmio_read32;

const XECP_ID_SUPPORTED_PROTOCOL: u32 = 2;
/// The capability's first three dwords: revision, name, and port range.
const SUPPORTED_PROTOCOL_BYTES: u64 = 12;
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

    /// Read every Supported Protocol capability through `read` (an offset
    /// from the register base), as `walk_ext_caps` bounds it.
    pub fn parse(hccparams1: u32, mapped_len: u64, read: impl Fn(u64) -> u32) -> Self {
        let mut out = Self::unknown();
        walk_ext_caps(hccparams1, mapped_len, SUPPORTED_PROTOCOL_BYTES, &read, |off, dw0| {
            if dw0 & 0xFF == XECP_ID_SUPPORTED_PROTOCOL {
                out.record_capability(dw0, read(off + 8));
            }
            true
        });
        out
    }

    /// The same, over the mapped registers at `mmio_base`.
    pub fn read(mmio_base: u64, mapped_len: u64) -> Self {
        let hccparams1 = mmio_read32(mmio_base + HCCPARAMS1);
        Self::parse(hccparams1, mapped_len, |off| mmio_read32(mmio_base + off))
    }

    fn record_capability(&mut self, dw0: u32, dw2: u32) {
        let major = (dw0 >> 24) as u8;
        let first = (dw2 & 0xFF) as u8;
        let count = ((dw2 >> 8) & 0xFF) as u8;
        self.record(major, first, count);
    }
}
