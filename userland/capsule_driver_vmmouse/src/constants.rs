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

//! The backdoor's absolute pointer (VMware's, and QEMU's vmmouse).

pub const CMD_GETVERSION: u32 = 10;
pub const CMD_ABSPOINTER_DATA: u32 = 39;
pub const CMD_ABSPOINTER_STATUS: u32 = 40;
pub const CMD_ABSPOINTER_COMMAND: u32 = 41;

/// The backdoor's magic, which GETVERSION hands back in EBX.
pub const MAGIC: u32 = 0x564D_5868;

pub const ENABLE: u32 = 0x4541_4552;
pub const DISABLE: u32 = 0x0000_00F5;
pub const REQUEST_ABSOLUTE: u32 = 0x5342_4152;

/// The word an enabled pointer queues first, to say what it is.
pub const VERSION_ID: u32 = 0x3442_554A;

/// STATUS: the pointer's error in the high half, its queued words in the low.
pub const STATUS_ERROR: u32 = 0xFFFF_0000;
pub const QUEUE_MASK: u32 = 0x0000_FFFF;

/// A packet is four words: flags and buttons, X, Y, the wheel.
pub const PACKET_WORDS: u32 = 4;
pub const RELATIVE_PACKET: u32 = 0x0001_0000;
pub const BUTTON_LEFT: u32 = 0x20;
pub const BUTTON_RIGHT: u32 = 0x10;
pub const BUTTON_MIDDLE: u32 = 0x08;

/// Both axes run from 0 to 0xFFFF over the whole guest display.
pub const RANGE_MAX: i32 = 0xFFFF;

/// How long the loop sleeps when the queue is empty.
pub const POLL_MS: u64 = 4;
/// The packets one look takes at most, so a host that keeps the queue full
/// cannot hold the driver in one look.
pub const PACKETS_PER_LOOK: usize = 16;
/// How long a pointer that would not come back is left before the next try.
pub const RETRY_MS: u64 = 1000;
