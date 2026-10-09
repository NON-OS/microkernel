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

//! rtw88 disables this chip's PCIe completion timeout by default
//! (pci.c, rtw_pci_phy_cfg: PCI_EXP_DEVCTL2_COMP_TMOUT_DIS for 8821C). Through
//! VT-d a read completion can come late, and a timed-out one stalls the
//! card's DMA. The broker lets a network function set exactly these bits.

use nonos_libc::{mk_pci_config_read, mk_pci_config_write};

const CAP_PTR: u32 = 0x34;
const CAP_ID_PCIE: i64 = 0x10;
const DEVCTL2: u32 = 0x28;
const COMP_TMOUT_DIS: u16 = 1 << 4;

/// Set Completion Timeout Disable. `false` when the function has no PCIe
/// capability or the write was refused.
pub fn disable_completion_timeout(device_id: u64, epoch: u64) -> bool {
    let Some(cap) = pcie_capability(device_id, epoch) else {
        return false;
    };
    let cur = mk_pci_config_read(device_id, epoch, cap + DEVCTL2, 2);
    if cur < 0 {
        return false;
    }
    let cur = cur as u16;
    if cur & COMP_TMOUT_DIS != 0 {
        return true;
    }
    mk_pci_config_write(device_id, epoch, cap + DEVCTL2, cur | COMP_TMOUT_DIS) == 0
}

fn pcie_capability(device_id: u64, epoch: u64) -> Option<u32> {
    let mut ptr = mk_pci_config_read(device_id, epoch, CAP_PTR, 1);
    // 48 capabilities at most fit in the 192 bytes after the header.
    for _ in 0..48 {
        let at = (ptr.max(0) as u32) & 0xFC;
        if ptr < 0 || at == 0 {
            return None;
        }
        if mk_pci_config_read(device_id, epoch, at, 1) == CAP_ID_PCIE {
            return Some(at);
        }
        ptr = mk_pci_config_read(device_id, epoch, at + 1, 1);
    }
    None
}
