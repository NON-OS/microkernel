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

//! Apollo Lake's DMA FIFO threshold, lowered as Linux lowers it.

use crate::constants::{PCI_VENDOR_INTEL, VS_EM4L};
use crate::regs::Regs;

const APL: u16 = 0x5a98;

/// `bxt_reduce_dma_latency`: keep only VS_EM4L bits 21:20, when the register
/// lies inside the `window` the broker mapped.
pub fn reduce_dma_latency(regs: Regs, vendor: u16, device: u16, window: u64) {
    if vendor != PCI_VENDOR_INTEL || device != APL || window < (VS_EM4L as u64 + 4) {
        return;
    }
    // SAFETY: VS_EM4L is 4-byte aligned and, as checked above, inside the
    // mapped window.
    unsafe {
        let v = regs.r32(VS_EM4L);
        regs.w32(VS_EM4L, v & (0x3 << 20));
    }
}
