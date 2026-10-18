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

pub(crate) const IO_QID: u16 = 1;
pub(super) const IO_ENTRIES: u16 = 8;
pub(super) const SQ_BYTES: u64 = 4096;
pub(super) const CQ_BYTES: u64 = 4096;
pub(super) const PRP_LIST_BYTES: u64 = 4096;
pub const SECTOR_SIZE: usize = 512;
pub const MAX_SECTORS: u32 = 64;
pub(super) const DATA_BYTES: u64 = MAX_SECTORS as u64 * SECTOR_SIZE as u64;
pub(super) const PAGE: u64 = 4096;
/// How long one read, write or flush may take, as Linux's nvme io_timeout:
/// a DRAM-less drive with no host memory buffer can stall for seconds while it
/// writes out its mapping tables in the middle of a long install.
pub(super) const COMPLETION_TIMEOUT_MS: u64 = 30_000;
/// How long the kernel waits for a reply from when it queued the request,
/// SLOW_BUDGET_MS in src/services/lifecycle/reply_wait.rs. An answer after
/// that is lost: the kernel hears a transport failure, not a status.
pub(super) const KERNEL_WAIT_MS: u64 = 35_000;
/// A read, write or flush is answered within this long of being taken, to
/// a 2 ms park, and one already queued as a recovery ran within this long
/// of the reply before it, or KERNEL_WAIT_MS on for each refused (bound):
/// 2 s inside the kernel's wait, for the reply to reach it.
pub(super) const REPLY_BY_MS: u64 = KERNEL_WAIT_MS - 2_000;
