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

//! The one DMA grant, page by page: the rings state, the request ring, the
//! completion ring, the sense buffer, then the data buffer. One request is
//! out at a time.

pub const PAGE: usize = 4096;
pub const STATE_OFF: usize = 0;
pub const REQ_RING_OFF: usize = PAGE;
pub const CMP_RING_OFF: usize = 2 * PAGE;
pub const SENSE_OFF: usize = 3 * PAGE;
pub const SENSE_BYTES: u32 = 96;
pub const DATA_OFF: usize = 4 * PAGE;
/// 64 KiB: the largest transfer, the same as every disk driver's.
pub const DATA_BYTES: usize = 0x1_0000;
pub const GRANT_BYTES: u64 = (DATA_OFF + DATA_BYTES) as u64;

/// One page each: 32 request descriptors, 128 completion descriptors.
pub const REQ_LOG2: u32 = 5;
pub const CMP_LOG2: u32 = 7;
const _: () = assert!((1 << REQ_LOG2) * super::regs::REQ_BYTES == PAGE);
const _: () = assert!((1 << CMP_LOG2) * super::regs::CMP_BYTES == PAGE);
