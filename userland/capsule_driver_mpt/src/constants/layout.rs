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

//! The one DMA grant, below 4 GiB (request frames, reply frames and the
//! sense buffer are named by 32-bit addresses with a zero high half): the
//! request frame, the reply frames, the sense buffer, then the data buffer.
//! One request is out at a time.

pub const REQUEST_OFF: usize = 0x000;
pub const REQUEST_BYTES: usize = 128;
pub const REPLY_OFF: usize = 0x400;
pub const REPLY_BYTES: usize = 128;
/// Reply frames handed to the reply free FIFO: an address reply takes one,
/// and each is given back once read.
pub const REPLY_FRAMES: usize = 8;
pub const SENSE_OFF: usize = 0x800;
pub const SENSE_BYTES: u8 = 96;
pub const DATA_OFF: usize = 0x1000;
/// 64 KiB: the largest transfer, the same as every disk driver's.
pub const DATA_BYTES: usize = 0x1_0000;
pub const GRANT_BYTES: u64 = (DATA_OFF + DATA_BYTES) as u64;

const _: () = assert!(REQUEST_OFF + REQUEST_BYTES <= REPLY_OFF);
const _: () = assert!(REPLY_OFF + REPLY_FRAMES * REPLY_BYTES <= SENSE_OFF);
