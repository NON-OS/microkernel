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


//! The cache maintenance the driver asks of libc where the CPU does not snoop
//! for it (every architecture but x86_64). The proofs run on a host whose
//! memory is one coherent buffer, so there is nothing to clean or invalidate;
//! the call only has to exist for the included driver file to build there.

/// Make a DMA range visible to the CPU: nothing to do over host memory.
///
/// # Safety
/// The same contract as libc's: `ptr .. ptr + len` lies in one DMA grant.
pub unsafe fn mk_dma_sync_for_cpu(_ptr: *const u8, _len: u64) {}
