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

//! Retire over the broker's real tables, page tables and config space.

extern crate alloc;

use alloc::vec::Vec;

use super::retire::Retire;
use crate::hardware::broker::dma::{free_taken, take_for_device as take_dma};
use crate::hardware::broker::mmio::{revoke_taken, take_for_device as take_mmio};
use crate::hardware::broker::{DmaGrant, MmioGrant};

/// One device leaving `pid`. `sleeper` is the caller in its own syscall,
/// whose reset waits may sleep (reset_wait); `in_context` when `pid`'s
/// page tables are the active ones, so its DMA pages unmap here too.
pub(super) struct Leaving {
    pub(super) pid: u32,
    pub(super) device_id: u64,
    pub(super) sleeper: Option<u32>,
    pub(super) in_context: bool,
    pub(super) mmio: Vec<MmioGrant>,
    pub(super) dma: Vec<DmaGrant>,
}

impl Leaving {
    pub(super) fn new(pid: u32, device_id: u64, sleeper: Option<u32>, in_context: bool) -> Self {
        Self { pid, device_id, sleeper, in_context, mmio: Vec::new(), dma: Vec::new() }
    }
}

impl Retire for Leaving {
    fn stop_bus_master(&mut self) {
        super::quiesce::stop_bus_master(self.device_id);
    }
    fn take_grants(&mut self) {
        self.mmio = take_mmio(self.pid, self.device_id);
        self.dma = take_dma(self.pid, self.device_id);
    }
    fn revoke_mmio(&mut self) {
        revoke_taken(self.pid, &self.mmio);
    }
    fn reset(&mut self) {
        super::reset::at_release(self.device_id, self.sleeper);
    }
    fn free_frames(&mut self) {
        free_taken(&self.dma, self.in_context);
    }
}
