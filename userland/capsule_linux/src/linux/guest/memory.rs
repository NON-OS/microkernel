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

//! A guest's memory as a call reads and writes it. A call written against
//! this rather than against a guest runs unchanged in the host proofs, over
//! a buffer that stands in for the guest, so the order of its reads and
//! writes, and what it does with a pointer that faults, can be proved.

use alloc::vec::Vec;

pub trait Memory {
    /// `len` bytes at `at`, or None when any of them cannot be read.
    fn read_at(&self, at: u64, len: usize) -> Option<Vec<u8>>;
    /// Bytes written, or a negative value on the first failure.
    fn write_at(&self, at: u64, bytes: &[u8]) -> i64;
}
