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

//! Synthetic config spaces: a builder, and the layout QEMU 8.2 gives a
//! modern-only virtio function (disable-legacy=on, iommu_platform=on).
//!
//! QEMU (hw/virtio/virtio-pci.c) places common, ISR, device and notify
//! structures at 0x0000, 0x1000, 0x2000 and 0x3000 of a 16 KiB 64-bit
//! memory BAR4 with a notify multiplier of 4, adds the config-access
//! capability, and gives MSI-X an exclusive BAR1 with the table at 0 and
//! the PBA at 0x800. Each capability is prepended to the list, so the walk
//! meets them in reverse: MSI-X first, common configuration last.

use crate::caps::{CFG_COMMON, CFG_DEVICE, CFG_ISR, CFG_NOTIFY, CFG_PCI};
use crate::pci::{BarInfo, Bars, ConfigSpace, CONFIG_SPACE_LEN};

pub const CAP_VENDOR: u8 = 0x09;
pub const CAP_MSIX: u8 = 0x11;
pub const MODERN_NET: u16 = 0x1041;
pub const TRANSITIONAL_NET: u16 = 0x1000;
pub const QEMU_MODERN_BAR: u8 = 4;
pub const QEMU_MSIX_BAR: u8 = 1;
pub const QEMU_MODERN_BAR_SIZE: u64 = 0x4000;

pub struct Space {
    bytes: [u8; CONFIG_SPACE_LEN],
}

impl Space {
    /// Vendor 0x1AF4, `device`, the Capabilities List bit set, head `head`.
    pub fn new(device: u16, head: u8) -> Self {
        let mut s = Self { bytes: [0; CONFIG_SPACE_LEN] };
        s.put16(0x00, 0x1AF4);
        s.put16(0x02, device);
        s.put16(0x06, 0x0010);
        s.put8(0x34, head);
        s
    }

    pub fn without_cap_list(mut self) -> Self {
        self.put16(0x06, 0);
        self
    }

    /// Bytes past the end of config space are dropped, so truncated
    /// capabilities near the end can be built.
    pub fn put8(&mut self, off: usize, v: u8) -> &mut Self {
        if let Some(b) = self.bytes.get_mut(off) {
            *b = v;
        }
        self
    }

    pub fn put16(&mut self, off: usize, v: u16) -> &mut Self {
        for (i, b) in v.to_le_bytes().iter().enumerate() {
            self.put8(off + i, *b);
        }
        self
    }

    pub fn put32(&mut self, off: usize, v: u32) -> &mut Self {
        for (i, b) in v.to_le_bytes().iter().enumerate() {
            self.put8(off + i, *b);
        }
        self
    }

    /// A virtio vendor capability with an explicit cap_len.
    #[allow(clippy::too_many_arguments)]
    pub fn raw_cap(
        &mut self,
        at: u8,
        next: u8,
        cap_len: u8,
        cfg_type: u8,
        bar: u8,
        offset: u32,
        length: u32,
    ) -> &mut Self {
        let a = at as usize;
        self.put8(a, CAP_VENDOR).put8(a + 1, next).put8(a + 2, cap_len).put8(a + 3, cfg_type);
        self.put8(a + 4, bar).put32(a + 8, offset).put32(a + 12, length)
    }

    /// struct virtio_pci_cap, 16 bytes.
    pub fn cap(
        &mut self,
        at: u8,
        next: u8,
        cfg_type: u8,
        bar: u8,
        offset: u32,
        length: u32,
    ) -> &mut Self {
        self.raw_cap(at, next, 16, cfg_type, bar, offset, length)
    }

    /// struct virtio_pci_notify_cap, 20 bytes.
    pub fn notify(
        &mut self,
        at: u8,
        next: u8,
        bar: u8,
        offset: u32,
        length: u32,
        mult: u32,
    ) -> &mut Self {
        self.raw_cap(at, next, 20, CFG_NOTIFY, bar, offset, length).put32(at as usize + 16, mult)
    }

    /// An MSI-X capability with `entries` vectors.
    #[allow(clippy::too_many_arguments)]
    pub fn msix(
        &mut self,
        at: u8,
        next: u8,
        entries: u16,
        table_bar: u8,
        table_off: u32,
        pba_bar: u8,
        pba_off: u32,
    ) -> &mut Self {
        let a = at as usize;
        self.put8(a, CAP_MSIX).put8(a + 1, next).put16(a + 2, entries - 1);
        self.put32(a + 4, table_off | table_bar as u32).put32(a + 8, pba_off | pba_bar as u32)
    }

    pub fn cfg(&self) -> ConfigSpace {
        ConfigSpace::from_bytes(self.bytes)
    }

    pub fn bytes(&self) -> [u8; CONFIG_SPACE_LEN] {
        self.bytes
    }
}

/// BAR1 holds the MSI-X table alone; BAR4 (64-bit, so BAR5 is its upper
/// half and listed absent) holds the four virtio structures.
pub fn qemu_modern_bars() -> Bars {
    [
        BarInfo::ABSENT,
        BarInfo::mmio(0x1000),
        BarInfo::ABSENT,
        BarInfo::ABSENT,
        BarInfo::mmio(QEMU_MODERN_BAR_SIZE),
        BarInfo::ABSENT,
    ]
}

/// The same function in transitional mode: a legacy I/O BAR0 as well.
pub fn qemu_transitional_bars() -> Bars {
    let mut bars = qemu_modern_bars();
    bars[0] = BarInfo::io(0x20);
    bars
}

pub fn qemu_modern(device: u16) -> Space {
    let mut s = Space::new(device, 0x98);
    s.msix(0x98, 0x84, 4, QEMU_MSIX_BAR, 0x0000, QEMU_MSIX_BAR, 0x0800);
    s.raw_cap(0x84, 0x70, 20, CFG_PCI, 0, 0, 0);
    s.notify(0x70, 0x60, QEMU_MODERN_BAR, 0x3000, 0x1000, 4);
    s.cap(0x60, 0x50, CFG_DEVICE, QEMU_MODERN_BAR, 0x2000, 0x1000);
    s.cap(0x50, 0x40, CFG_ISR, QEMU_MODERN_BAR, 0x1000, 0x1000);
    s.cap(0x40, 0x00, CFG_COMMON, QEMU_MODERN_BAR, 0x0000, 0x1000);
    s
}
