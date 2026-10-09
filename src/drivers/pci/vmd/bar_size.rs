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

//! Sizing the memory BARs of a function behind a VMD.

use super::super::config::ConfigSpace;
use super::super::types::PciBar;
use crate::memory::addr::PhysAddr;

const CMD_IO_MEM: u16 = 0x3;

/// Size the memory BARs the assignment placed. Decode is off while each is
/// probed with ones, so the device never answers at the probe pattern.
pub(super) fn bars(config: &ConfigSpace) -> [PciBar; 6] {
    let mut out = [PciBar::NotPresent; 6];
    let Ok(command) = config.command() else {
        return out;
    };
    let _ = config.set_command(command & !CMD_IO_MEM);
    let mut index = 0u16;
    while index < 6 {
        let reg = 0x10 + index * 4;
        let original = config.read32(reg).unwrap_or(0);
        if original & 1 != 0 {
            index += 1;
            continue;
        }
        let wide = (original >> 1) & 0x3 == 2 && index < 5;
        let _ = config.write32(reg, 0xFFFF_FFFF);
        let low = config.read32(reg).unwrap_or(0) & !0xF;
        let _ = config.write32(reg, original);
        let (high_orig, high_mask) = if wide {
            let orig = config.read32(reg + 4).unwrap_or(0);
            let _ = config.write32(reg + 4, 0xFFFF_FFFF);
            let mask = config.read32(reg + 4).unwrap_or(0);
            let _ = config.write32(reg + 4, orig);
            (orig, mask)
        } else {
            (0, 0xFFFF_FFFF)
        };
        let address = ((high_orig as u64) << 32) | (original & !0xF) as u64;
        let size = (!(((high_mask as u64) << 32) | low as u64)).wrapping_add(1);
        if low != 0 && address != 0 && size != 0 {
            let prefetchable = original & 0x8 != 0;
            out[index as usize] = if wide {
                PciBar::Memory64 { address: PhysAddr::new(address), size, prefetchable }
            } else {
                PciBar::Memory32 { address: PhysAddr::new(address), size, prefetchable }
            };
        }
        index += if wide { 2 } else { 1 };
    }
    let _ = config.set_command(command);
    out
}
