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

//! The ADMA2 descriptor table for one data phase, and the registers that
//! point the host at it.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::adma::{self, AdmaError, Width, TABLE_BYTES};
use super::super::regs::{
    ADMA_ADDRESS, ADMA_ADDRESS_HI, BLOCK_COUNT, BLOCK_SIZE, BLOCK_SIZE_512, HC_ADMA2_32,
    HC_ADMA2_64, HC_DMA_MASK, HOST_CONTROL,
};
use super::cmd::Data;
use super::engine::Host;
use super::limits::BLOCK;

impl<M: Mmio, C: Clock, L: Log> Host<M, C, L> {
    /// Write the descriptor table for `data` and point the host at it.
    pub(super) fn program_data(&self, data: &Data) -> EmmcResult<()> {
        let len = data.blocks as usize * BLOCK;
        if data.blocks == 0 || len > data.buf.len {
            return Err(EmmcError::OutOfRange);
        }
        let width = adma::width_for(
            self.table.bus,
            self.table.len as u64,
            data.buf.bus,
            len as u64,
            self.caps.dma64(),
        )
        .ok_or(EmmcError::DmaAddress)?;
        if !self.table.bus.is_multiple_of(width.align()) {
            return Err(EmmcError::DmaAddress);
        }
        let mut t = [0u8; TABLE_BYTES];
        let n = adma::build(&mut t, width, data.buf.bus, len).map_err(|e| match e {
            AdmaError::Align | AdmaError::Reach => EmmcError::DmaAddress,
            AdmaError::Length | AdmaError::Room => EmmcError::OutOfRange,
        })?;
        if !self.table.put(0, &t[..n * width.desc_len()]) {
            return Err(EmmcError::OutOfRange);
        }
        let sel = match width {
            Width::A32 => HC_ADMA2_32,
            Width::A64 => HC_ADMA2_64,
        };
        self.io.w8(HOST_CONTROL, (self.io.r8(HOST_CONTROL) & !HC_DMA_MASK) | sel);
        self.io.w32(ADMA_ADDRESS, self.table.bus as u32);
        if width == Width::A64 {
            self.io.w32(ADMA_ADDRESS_HI, (self.table.bus >> 32) as u32);
        }
        self.io.w16(BLOCK_SIZE, BLOCK_SIZE_512);
        self.io.w16(BLOCK_COUNT, data.blocks);
        Ok(())
    }
}
