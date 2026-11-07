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

//! What the broker needs from the capsule it runs in.

use crate::errno::Errno;

/// Numbered objects on the data volume: the kernel's MkUserObj calls. An
/// object is written whole and read by range; `put` over an existing number
/// replaces it.
pub trait Objects {
    fn put(&mut self, id: u64, bytes: &[u8]) -> Result<(), Errno>;
    /// Bytes read into `buf` from `offset`; 0 at the end.
    fn read(&mut self, id: u64, offset: u64, buf: &mut [u8]) -> Result<usize, Errno>;
    /// Its length; ENOENT when there is none.
    fn size(&mut self, id: u64) -> Result<u64, Errno>;
    /// Removed; ENOENT when there was none.
    fn remove(&mut self, id: u64) -> Result<(), Errno>;
}

/// The facts about time and clients the kernel answers.
pub trait Host {
    /// Wall-clock nanoseconds since the Unix epoch, for file times.
    fn now_ns(&self) -> u64;
    /// Monotonic nanoseconds, for when an owed commit is due.
    fn mono_ns(&self) -> u64;
    /// Whether `pid` may use the broker at all.
    fn may_use(&self, pid: u32) -> bool;
    /// Whether `pid` still runs; a client that ended has its files closed.
    fn alive(&self, pid: u32) -> bool;
}
