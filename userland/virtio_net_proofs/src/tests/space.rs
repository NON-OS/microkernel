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

//! Config space behind a broker in memory: a transitional virtio-net with
//! or without the virtio 1.0 structures, or one whose reads are refused.

use nonos_virtio::{BarInfo, Bars, Broker, MmioGrant};

/// Config space as the broker would read it, or every read refused.
pub struct Space(pub Option<[u8; 256]>);

// SAFETY: `mmio_map` never returns Ok, so no mapping is ever promised.
unsafe impl Broker for Space {
    fn config_read32(&mut self, off: u32) -> Option<u32> {
        let b = self.0?;
        let at = off as usize;
        Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
    }
    fn mmio_map(&mut self, _: u8, _: u64, _: u64) -> Result<MmioGrant, i64> {
        Err(-1)
    }
    fn mmio_unmap(&mut self, _: u64) -> bool {
        false
    }
}

/// A transitional virtio-net (0x1000), with QEMU's common, notify and
/// device structures in BAR4 when `modern`.
pub fn transitional(modern: bool) -> Space {
    let mut b = [0u8; 256];
    b[..4].copy_from_slice(&[0xF4, 0x1A, 0x00, 0x10]);
    if modern {
        b[0x06] = 0x10;
        b[0x34] = 0x40;
        // cap_vndr, next, cap_len, cfg_type, bar, then offset and length.
        b[0x40..0x48].copy_from_slice(&[0x09, 0x50, 16, 1, 4, 0, 0, 0]);
        b[0x48..0x50].copy_from_slice(&[0, 0, 0, 0, 0x38, 0, 0, 0]);
        b[0x50..0x58].copy_from_slice(&[0x09, 0x64, 20, 2, 4, 0, 0, 0]);
        b[0x58..0x64].copy_from_slice(&[0, 0x30, 0, 0, 0, 0x10, 0, 0, 4, 0, 0, 0]);
        b[0x64..0x6C].copy_from_slice(&[0x09, 0x00, 16, 4, 4, 0, 0, 0]);
        b[0x6C..0x74].copy_from_slice(&[0, 0x20, 0, 0, 0, 0x10, 0, 0]);
    }
    Space(Some(b))
}

impl Space {
    /// The same function with its `cfg_type` structure in the unmappable I/O BAR.
    pub fn in_io_bar(mut self, cfg_type: u8) -> Self {
        let b = self.0.as_mut().expect("a readable config space");
        let at = [0x40, 0x50, 0x64].into_iter().find(|&at| b[at + 3] == cfg_type);
        b[at.expect("a structure of that type") + 4] = 0;
        self
    }
}

pub const BARS: Bars = {
    let mut bars = [BarInfo::ABSENT; 6];
    bars[0] = BarInfo::io(0x20);
    bars[4] = BarInfo::mmio(0x4000);
    bars
};
