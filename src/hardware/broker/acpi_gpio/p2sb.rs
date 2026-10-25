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

//! SBREG_BAR, the base of the PCH sideband register space the GPIO
//! communities of Sunrise Point and later sit in. The P2SB bridge at 00:1f.1
//! holds it in BAR0 and firmware usually hides the bridge, so this unhides
//! it, reads BAR0 and hides it again, and never touches a function there
//! that is not a P2SB. The hide control (P2SBC at 0xE0, HIDE bit 8) is
//! recalled from the PCH datasheets (volume 2, P2SB): unverified.

use crate::arch::firmware_table::devices::sbreg_from_bar0;
use crate::bus::pci::{pci_read16, pci_read32, pci_write32};
use crate::sys::serial;

const DEV: u8 = 31;
const FUNC: u8 = 1;
// The P2SB control register and its hide bit (PCH datasheet; recalled, unverified).
const P2SBC: u8 = 0xE0;
const P2SBC_HIDE: u32 = 1 << 8;
// A P2SB is class 05h subclass 80h (PCI_CLASS_MEMORY_OTHER).
const CLASS_MEMORY_OTHER: u16 = 0x0580;
const INTEL: u16 = 0x8086;

/// SBREG_BAR, or None when 00:1f.1 is something else or the BAR is unset.
/// Runs while the broker table is seeded, before any capsule can reach
/// config space, so the bridge is never seen unhidden by anyone else.
pub(super) fn sbreg_bar() -> Option<u64> {
    let class = pci_read16(0, DEV, FUNC, 0x0A);
    if class != u16::MAX && class != CLASS_MEMORY_OTHER {
        return None;
    }
    // A hidden bridge reads all ones, which carries the hide bit too.
    let hidden = pci_read32(0, DEV, FUNC, P2SBC) & P2SBC_HIDE != 0;
    // Said before the write, so a machine that stops here names the step.
    serial::println(if hidden {
        b"[GPIO] P2SB at 00:1f.1 hidden by firmware, unhiding it to read SBREG_BAR"
    } else {
        b"[GPIO] P2SB at 00:1f.1 visible, reading SBREG_BAR"
    });
    if hidden {
        pci_write32(0, DEV, FUNC, P2SBC, 0);
    }
    let vendor = pci_read16(0, DEV, FUNC, 0x00);
    let lo = pci_read32(0, DEV, FUNC, 0x10);
    let hi = pci_read32(0, DEV, FUNC, 0x14);
    if hidden {
        pci_write32(0, DEV, FUNC, P2SBC, P2SBC_HIDE);
        serial::println(b"[GPIO] P2SB hidden again");
    }
    if vendor != INTEL {
        return None;
    }
    sbreg_from_bar0(lo, hi)
}
