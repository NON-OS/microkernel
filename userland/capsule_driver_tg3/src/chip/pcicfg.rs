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

//! The configuration words tg3 writes, as pure functions of what the
//! register holds now, so each can be held to the broker's rule. Linux
//! writes whole dwords; the broker takes the low 16 bits, which hold every
//! bit these change.

use crate::constants::pcicfg::{
    MISC_HOST_CTRL, MISC_HOST_CTRL_CONTROL, PCISTATE_ROM_ENABLE, PCISTATE_ROM_RETRY_ENABLE,
    PCI_EXP_DEVCTL_NOSNOOP_EN, PCI_EXP_DEVCTL_RELAX_EN, PCI_EXP_DEVSTA_ERRORS,
};

/// TG3PCI_MISC_HOST_CTRL: tp->misc_host_ctrl's control bits in place of
/// whatever the reset left (tg3_restore_pci_state).
pub fn host_control(current: u16) -> u16 {
    (current & !MISC_HOST_CTRL_CONTROL) | MISC_HOST_CTRL as u16
}

/// TG3PCI_PCISTATE: ROM enable and ROM retry on. Linux writes the dword as
/// just those two; the other low bits are read-only or zero after a reset,
/// and are kept as read so the broker sees only its two bits move.
pub fn pci_state(current: u16) -> u16 {
    current | PCISTATE_ROM_ENABLE | PCISTATE_ROM_RETRY_ENABLE
}

/// PCI Express Device Control with Relaxed Ordering and No Snoop cleared;
/// a part with a CPMU keeps its payload size (tg3_chip_reset).
pub fn device_control(current: u16) -> u16 {
    current & !(PCI_EXP_DEVCTL_RELAX_EN | PCI_EXP_DEVCTL_NOSNOOP_EN)
}

/// PCI Express Device Status with the four error bits written to clear.
pub fn device_status(current: u16) -> u16 {
    current | PCI_EXP_DEVSTA_ERRORS
}
