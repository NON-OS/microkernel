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

//! The data buffer between a request and the controller. A client is handed
//! the buffer's bytes only after the read's own completion came back clean,
//! and the buffer is filled only once no command given up on can still be
//! moving data through it.

use super::constants::DATA_BYTES;
use super::queue::IoQueue;
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

impl IoQueue {
    /// Bytes `sectors` blocks take, refused past the data buffer.
    pub(super) fn bytes(&self, sectors: u32) -> NvmeResult<usize> {
        let bytes = sectors as u64 * self.lba_size as u64;
        if bytes == 0 || bytes > DATA_BYTES {
            return Err(NvmeError::InvalidTransfer);
        }
        Ok(bytes as usize)
    }

    /// Read `sectors` blocks at `lba` into the front of `dst`.
    pub fn read_into(
        &mut self,
        regs: Regs,
        lba: u64,
        sectors: u32,
        dst: &mut [u8],
    ) -> NvmeResult<()> {
        let bytes = self.bytes(sectors)?;
        let dst = dst.get_mut(..bytes).ok_or(NvmeError::InvalidTransfer)?;
        self.transfer(regs, lba, sectors, false)?;
        // SAFETY: the data buffer is DATA_BYTES long, at least `bytes`, and
        // the read's own completion was seen, so the controller is done
        // writing it.
        let src = unsafe { core::slice::from_raw_parts(self.data.user_va() as *const u8, bytes) };
        dst.copy_from_slice(src);
        Ok(())
    }

    /// Write `src`, exactly `sectors` blocks, at `lba`.
    pub fn write_from(&mut self, regs: Regs, lba: u64, sectors: u32, src: &[u8]) -> NvmeResult<()> {
        if src.len() != self.bytes(sectors)? {
            return Err(NvmeError::InvalidTransfer);
        }
        // The buffer may still be the controller's, for a command given up on.
        self.settle(regs)?;
        // SAFETY: the data buffer is DATA_BYTES long, at least `src.len()`,
        // and no command is out, so the controller is not reading it.
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), self.data.user_va() as *mut u8, src.len())
        };
        self.transfer(regs, lba, sectors, true)
    }
}
