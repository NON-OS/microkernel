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

//! The one DMA grant: the request queue's split ring in the first page, the
//! request header and its response in the second, then the data buffer. One
//! request is out at a time, so one descriptor chain is enough.

/// The ring the driver asks for; a smaller one down to `QUEUE_MIN` works.
pub const QUEUE_MAX: u16 = 16;
/// A chain is the header, the response and at most one data buffer.
pub const QUEUE_MIN: u16 = 4;
pub const DESC_OFF: usize = 0;
pub const AVAIL_OFF: usize = 0x400;
pub const USED_OFF: usize = 0x800;
pub const REQ_OFF: usize = 0x1000;
pub const RESP_OFF: usize = 0x1100;
pub const DATA_OFF: usize = 0x2000;
/// 64 KiB: the largest transfer, the same as every disk driver's.
pub const DATA_BYTES: usize = 0x1_0000;
pub const GRANT_BYTES: u64 = (DATA_OFF + DATA_BYTES) as u64;

/// VIRTQ_DESC_F_NEXT and VIRTQ_DESC_F_WRITE.
pub const DESC_F_NEXT: u16 = 1;
pub const DESC_F_WRITE: u16 = 2;
/// VIRTQ_AVAIL_F_NO_INTERRUPT: the driver polls.
pub const AVAIL_F_NO_INTERRUPT: u16 = 1;

const Q: usize = QUEUE_MAX as usize;
const _: () = assert!(DESC_OFF + Q * 16 <= AVAIL_OFF);
const _: () = assert!(AVAIL_OFF + 6 + Q * 2 <= USED_OFF && USED_OFF.is_multiple_of(4));
const _: () = assert!(USED_OFF + 6 + Q * 8 <= REQ_OFF);
const _: () = assert!(REQ_OFF + super::scsi::REQ_LEN <= RESP_OFF);
const _: () = assert!(RESP_OFF + super::scsi::RESP_LEN <= DATA_OFF);
