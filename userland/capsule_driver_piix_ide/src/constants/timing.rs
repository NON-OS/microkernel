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

//! How long the driver waits on a device. A virtual disk answers at once;
//! these bound a host that stalls, and a slot whose device never settles.

/// SRST held, then released before BSY is read (ATA: at least 5 us, then
/// 2 ms).
pub const RESET_HOLD_MS: u64 = 1;
pub const RESET_RELEASE_MS: u64 = 2;
/// A channel whose devices are still busy this long after reset is skipped.
pub const RESET_READY_MS: u64 = 5_000;
/// Busy and DRQ clear on the selected device before a command is written.
pub const SELECT_MS: u64 = 1_000;
/// IDENTIFY DEVICE's data ready.
pub const IDENTIFY_MS: u64 = 2_000;
/// One transfer or one cache flush. Shorter than the client's own bound, so
/// the driver is the one that says why a request failed.
pub const COMMAND_MS: u64 = 10_000;
/// From the disk letting go to the engine going inactive: the time it
/// takes to empty its FIFO into memory.
pub const DRAIN_MS: u64 = 5;
/// Looks taken with a yield between them before a wait starts sleeping: a
/// virtual disk often completes within them, and a millisecond sleep per
/// request would cap the disk near a thousand requests a second.
pub const QUICK_LOOKS: u32 = 64;
