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

//! The one DMA grant, page by page: the guest's ring (32 pages, its header
//! page first), the host's ring (the same), then the data buffer. 128 KiB a
//! ring, as Linux's storvsc_ringbuffer_size. One request is out at a time.

use nonos_vmbus::Layout;

pub const PAGE: usize = 4096;
pub const RINGS: Layout = Layout { out_pages: 32, in_pages: 32 };
pub const RING_BYTES: usize = 64 * PAGE;
pub const DATA_OFF: usize = RING_BYTES;
/// 64 KiB: the largest transfer, the same as every disk driver's.
pub const DATA_BYTES: usize = 0x1_0000;
pub const DATA_PAGES: usize = DATA_BYTES / PAGE;
pub const GRANT_BYTES: u64 = (DATA_OFF + DATA_BYTES) as u64;

const _: () = assert!(RINGS.out_pages as usize + RINGS.in_pages as usize == RING_BYTES / PAGE);
