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

//! How long the device may take, by the clock (plan/50 C7). Every wait here
//! is measured against these, never against a count of wakes or reads.

/// A read or write the device has not answered in this long is taken to
/// have stopped. QEMU answers in microseconds, a loaded host disk in tens
/// of milliseconds.
pub const REQUEST_MS: u64 = 5_000;
/// A flush waits for the host to put everything down, which on a busy
/// host takes far longer than one read or write. The client's call
/// timeout (nonos_blk_client wire/call.rs) is longer than this and a reset.
pub const FLUSH_MS: u64 = 30_000;
/// The longest one sleep on the interrupt before the used ring is looked
/// at again, so a completion whose interrupt was lost is found by then.
pub const WAIT_SLICE_MS: u64 = 100;
/// How long a device may take to read back zero after a reset.
pub const RESET_MS: u64 = 1_000;
