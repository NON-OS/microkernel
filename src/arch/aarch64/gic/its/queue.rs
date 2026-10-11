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

//! The ITS command queue: one page of 32-byte commands, each batch closed
//! by a SYNC and waited on, so the queue is empty whenever a batch starts
//! and a batch can never overrun it.

use super::encode::attrs::{cbaser, coherent};
use super::memory::{alloc, free, Region};
use super::regs::{Regs, CBASER, CWRITER};

pub(super) const BYTES: u64 = 0x1000;
pub(super) const SLOT: u64 = 32;
/// The most commands one batch may hold, its SYNC included, so the write
/// offset never catches the read offset.
pub(super) const MAX_BATCH: usize = (BYTES / SLOT) as usize - 1;

pub(super) struct Queue {
    pub(super) region: Region,
    /// Where the next command goes, and where `GITS_CWRITER` points.
    pub(super) write: u64,
    pub(super) coherent: bool,
    /// The redistributor every SYNC names: the collection's.
    pub(super) target: u64,
}

impl Queue {
    /// Hand the ITS its queue. Only while it is disabled: `GITS_CBASER`
    /// written to an enabled ITS is unpredictable.
    pub(super) fn new(regs: Regs, target: u64) -> Result<Self, &'static str> {
        let region = alloc(BYTES, BYTES).ok_or("no memory for the ITS command queue")?;
        regs.write64(CBASER, cbaser(region.phys, 1, true));
        let coherent = coherent(regs.read64(CBASER));
        if !coherent {
            regs.write64(CBASER, cbaser(region.phys, 1, false));
        }
        if regs.read64(CBASER) & !0xfff == 0 {
            free(region);
            return Err("the ITS took no command queue");
        }
        regs.write64(CWRITER, 0);
        Ok(Self { region, write: 0, coherent, target })
    }
}
