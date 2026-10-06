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

//! The driver's state once the reader is taken. Fields drop in order: the
//! DMA buffers go back before the device is released.

use super::handles::Handles;
use super::region::Region;
use crate::chip::Family;
use crate::hw::Regs;

pub struct Driver {
    pub regs: Regs,
    pub family: Family,
    pub device: u16,
    /// DUMMY_REG_RESET_0's low nibble (rts5227_get_ic_version).
    pub ic_version: u8,
    /// The command buffer, then the scatter-gather table (rtsx_resv_buf).
    pub resv: Region,
    /// Where card data lands.
    pub data: Region,
    /// The SSC clock last programmed, in MHz (pcr->cur_clock).
    pub cur_clock: u32,
    /// Held for its Drop, which gives the mapping and the device back.
    pub _handles: Handles,
}
