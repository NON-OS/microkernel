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
//! Reading the Supported Protocol capabilities into `PortProtocols`.

use super::port_protocols::PortProtocols;
use super::xecp::walk_ext_caps;
use crate::constants::HCCPARAMS1;
use crate::regs::mmio_read32;

const XECP_ID_SUPPORTED_PROTOCOL: u32 = 2;
/// The capability's first three dwords: revision, name, and port range.
const SUPPORTED_PROTOCOL_BYTES: u64 = 12;

impl PortProtocols {
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
        // SAFETY: `mmio_base` starts the mapped window, and the capability registers are its first
        // 32 bytes.
        let hccparams1 = unsafe { mmio_read32(mmio_base + HCCPARAMS1) };
        // SAFETY: `walk_ext_caps` reads only offsets whose dwords it checked lie
        // inside the mapped window.
        let read = |off| unsafe { mmio_read32(mmio_base + off) };
        Self::parse(hccparams1, mapped_len, read)
    }

    fn record_capability(&mut self, dw0: u32, dw2: u32) {
        let major = (dw0 >> 24) as u8;
        let first = (dw2 & 0xFF) as u8;
        let count = ((dw2 >> 8) & 0xFF) as u8;
        self.record(major, first, count);
    }
}
