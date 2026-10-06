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

//! Turning off a claimed device's no-snoop requests. Enable No Snoop in the
//! PCIe Device Control register is 1 at reset (PCIe 5.0, 7.5.3.4, bit 11),
//! so any function may mark its DMA no-snoop. Such a write bypasses the CPU
//! caches: with no remapping unit forcing a snoop, a write-back grant then
//! holds a stale line over what the device wrote, and a ring the driver just
//! filled can be read from memory before its dirty lines get there. Cleared,
//! the function must snoop every request, so a write-back grant is coherent
//! on every machine. Linux leaves the bit to drivers; HD Audio's own snoop
//! bits (quirk_bits) stay the driver's.

use crate::drivers::pci::ConfigSpace;

const CAP_ID_PCIE: u8 = 0x10;
const DEVCTL: u16 = 0x08;
const DEVCTL_NOSNOOP_EN: u16 = 1 << 11;

pub(super) fn snoop_every_request(device_id: u64) {
    let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) else {
        return;
    };
    let cfg = ConfigSpace::new(handle.address);
    let Some(cap) = pcie_capability(&cfg) else {
        return;
    };
    let Ok(ctl) = cfg.read16(cap + DEVCTL) else {
        return;
    };
    if ctl & DEVCTL_NOSNOOP_EN == 0 {
        return;
    }
    let _ = cfg.write16(cap + DEVCTL, ctl & !DEVCTL_NOSNOOP_EN);
    let off = matches!(cfg.read16(cap + DEVCTL), Ok(v) if v & DEVCTL_NOSNOOP_EN == 0);
    crate::sys::serial::print(b"[DMA] device ");
    crate::sys::serial::print_hex(device_id);
    crate::sys::serial::println(if off { b" no-snoop off" } else { b" no-snoop STILL ON" });
}

/// The PCI Express capability's offset, walked through the function's own
/// configuration space so a device behind a VMD is reached too.
fn pcie_capability(cfg: &ConfigSpace) -> Option<u16> {
    if !cfg.has_capabilities().ok()? {
        return None;
    }
    let mut ptr = cfg.capabilities_pointer().ok()? & 0xFC;
    // 48 capabilities at most fit in the 192 bytes after the header.
    for _ in 0..48 {
        if ptr == 0 {
            return None;
        }
        if cfg.read8(ptr as u16).ok()? == CAP_ID_PCIE {
            return Some(ptr as u16);
        }
        ptr = cfg.read8(ptr as u16 + 1).ok()? & 0xFC;
    }
    None
}
