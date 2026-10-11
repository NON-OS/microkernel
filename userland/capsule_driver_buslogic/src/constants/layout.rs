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

//! The one DMA grant, below 4 GiB: a mailbox pair, the command control
//! block, the sense buffer, then the data buffer. One request is out at a
//! time, so one outgoing and one incoming mailbox are enough.

pub const MBO_OFF: usize = 0x000;
pub const MBI_OFF: usize = 0x008;
pub const CCB_OFF: usize = 0x040;
pub const SENSE_OFF: usize = 0x100;
pub const SENSE_BYTES: u8 = 96;
pub const DATA_OFF: usize = 0x1000;
/// 64 KiB: the largest transfer, the same as every disk driver's.
pub const DATA_BYTES: usize = 0x1_0000;
pub const GRANT_BYTES: u64 = (DATA_OFF + DATA_BYTES) as u64;
pub const MAILBOXES: u8 = 1;

// The incoming mailboxes follow the outgoing ones, as the adapter reads
// them from the one address it is given.
const _: () = assert!(MBI_OFF == MBO_OFF + MAILBOXES as usize * super::regs::MBOX_BYTES);
const _: () = assert!(CCB_OFF >= MBI_OFF + MAILBOXES as usize * super::regs::MBOX_BYTES);
const _: () = assert!(CCB_OFF + super::regs::CCB_BYTES <= SENSE_OFF);
