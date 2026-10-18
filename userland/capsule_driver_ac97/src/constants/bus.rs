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

//! The bus master window: the PCM out box at 0x10 and the global registers.

pub const PO_BDBAR: u16 = 0x10;
pub const PO_CIV: u16 = 0x14;
pub const PO_LVI: u16 = 0x15;
pub const PO_SR: u16 = 0x16;
pub const PO_CR: u16 = 0x1B;
pub const GLOB_CNT: u16 = 0x2C;
pub const GLOB_STA: u16 = 0x30;
pub const CAS: u16 = 0x34;

/// Status: the engine halted (DMA controller halted).
pub const SR_DCH: u16 = 1 << 0;
/// Status bits a write of one clears: last valid buffer, completion, FIFO.
pub const SR_CLEAR: u16 = (1 << 2) | (1 << 3) | (1 << 4);

/// Control: run the engine.
pub const CR_RPBM: u8 = 1 << 0;
/// Control: reset the box's registers; the part clears it when done.
pub const CR_RR: u8 = 1 << 1;

/// Global control: interrupts, cold reset released, warm reset, link off,
/// and the four and six channel modes.
pub const GC_GIE: u32 = 1 << 0;
pub const GC_COLD: u32 = 1 << 1;
pub const GC_ACLINK_OFF: u32 = 1 << 3;
pub const GC_PCM_246: u32 = 3 << 20;

/// Global status: the primary codec is ready.
pub const GS_PCR: u32 = 1 << 8;

/// Codec access semaphore: set while another access is in flight.
pub const CAS_BUSY: u8 = 1 << 0;
