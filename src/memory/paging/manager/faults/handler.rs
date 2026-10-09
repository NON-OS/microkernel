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

use crate::memory::addr::VirtAddr;

use super::super::core::PagingManager;
use super::super::pending_flush::PendingFlush;
use crate::memory::paging::constants::*;
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::paging::stats::PagingStatistics;

impl PagingManager {
    pub(in crate::memory::paging::manager) fn handle_page_fault(
        &mut self,
        virtual_addr: VirtAddr,
        error_code: u64,
        stats: &PagingStatistics,
    ) -> PagingResult<PendingFlush> {
        stats.record_page_fault();

        if error_code & PF_WRITE != 0 && error_code & PF_PRESENT != 0 {
            stats.record_cow_fault();
            return self.handle_cow_fault(virtual_addr, stats);
        }

        if error_code & PF_PRESENT == 0 {
            if error_code & PF_INSTRUCTION != 0 {
                return Err(PagingError::UnhandledPageFault);
            }
            stats.record_demand_load();
            let filled = self.handle_demand_fault(virtual_addr, stats);
            if filled.is_ok() {
                // Named only for a fill that happened: the guards inside refuse
                // the null page and the kernel half, and a refused fault is not
                // a fill.
                log_demand_fill(virtual_addr, error_code);
            }
            return filled;
        }

        Err(PagingError::UnhandledPageFault)
    }
}

// A demand fill puts a zeroed page where nothing was mapped. Named on the
// serial log so a fill that lands where code or a peer's page belonged is
// visible at the moment it happens, not only at the fault it causes later.
fn log_demand_fill(virtual_addr: VirtAddr, error_code: u64) {
    let pid = crate::process::current_pid().unwrap_or(0);
    crate::sys::serial::print(b"[PF] demand fill pid=");
    crate::sys::serial::print_hex(pid as u64);
    crate::sys::serial::print(b" va=");
    crate::sys::serial::print_hex(virtual_addr.as_u64());
    crate::sys::serial::print(b" err=");
    crate::sys::serial::print_hex(error_code);
    crate::sys::serial::println(b"");
}
