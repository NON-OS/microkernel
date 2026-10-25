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

//! The masked form of `MkPciConfigWrite`. The plain call ORs a Command
//! value holding only Memory Space, Bus Master or Interrupt Disable onto the
//! register, so it can turn those bits on but never off. This call names the
//! bits it changes, and each takes the value given: the way a driver stops
//! its device's DMA before a reset or D3, or clears Interrupt Disable to use
//! the legacy pin. The broker's rules are the same; a value bit outside
//! `mask` is refused.

use crate::syscall::{call_raw, N_MK_PCI_CONFIG_WRITE};

/// Where the change mask sits in the offset argument.
pub const MK_PCI_CFG_MASK_SHIFT: u32 = 16;

#[no_mangle]
pub extern "C" fn mk_pci_config_update(
    device_id: u64,
    claim_epoch: u64,
    offset: u32,
    mask: u16,
    value: u16,
) -> i64 {
    let arg = (offset & 0xFFFF) | ((mask as u32) << MK_PCI_CFG_MASK_SHIFT);
    call_raw(N_MK_PCI_CONFIG_WRITE, [device_id, claim_epoch, arg as u64, value as u64, 0, 0])
}
