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

//! The event queue: one coherent page holding the split ring and a buffer
//! of 8 bytes per descriptor. The offsets fit the largest ring taken, and
//! a smaller one the device asks for fits them too.

pub const QUEUE_MAX: u16 = 64;
pub const DESC_OFF: usize = 0;
pub const AVAIL_OFF: usize = 1024;
pub const USED_OFF: usize = 2048;
pub const EVENTS_OFF: usize = 3072;
pub const EVENT_BYTES: usize = 8;
pub const RING_BYTES: usize = 4096;
/// VIRTQ_DESC_F_WRITE: the device writes this buffer.
pub const DESC_F_WRITE: u16 = 2;
/// VIRTQ_AVAIL_F_NO_INTERRUPT: the driver polls.
pub const AVAIL_F_NO_INTERRUPT: u16 = 1;
/// How long the poll loop sleeps when no device had an event: a 250 Hz
/// pointer, and no core held while nothing moves.
pub const POLL_MS: u64 = 4;

const Q: usize = QUEUE_MAX as usize;
const _: () = assert!(DESC_OFF + Q * 16 <= AVAIL_OFF);
const _: () = assert!(AVAIL_OFF + 6 + Q * 2 <= USED_OFF && USED_OFF & 3 == 0);
const _: () = assert!(USED_OFF + 6 + Q * 8 <= EVENTS_OFF);
const _: () = assert!(EVENTS_OFF + Q * EVENT_BYTES <= RING_BYTES);
