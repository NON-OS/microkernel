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

//! The PCIe DMA burst sizes, set before the ring addresses as rtw88
//! `rtw_pci_reset_buf_desc` sets them (pci.c:403).

use crate::regs::Mmio;

/// `RTK_PCI_CTRL`: the byte at +3 holds the transmit and receive DMA burst
/// fields.
const REG_PCI_CTRL: usize = 0x0300;
/// The bits rtw88 sets there (pci.c:404, `tmp | 0xf7`).
const DMA_BURST: u8 = 0xF7;

/// Set the burst fields as rtw88 does, before any ring address is written.
pub fn set_dma_burst<M: Mmio>(mmio: &M) {
    let v = mmio.read8(REG_PCI_CTRL + 3);
    mmio.write8(REG_PCI_CTRL + 3, v | DMA_BURST);
}
