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

//! One span of a guest's address space, as this capsule laid it down.

#[derive(Clone, Copy)]
pub struct Region {
    pub at: u64,
    pub len: u64,
    pub write: bool,
    pub exec: bool,
    /// File bytes mapped without exec, so never proved: mprotect may not
    /// make them executable later.
    pub unproven: bool,
    /// False for a PROT_NONE reservation: address space taken, no frames yet.
    /// The kernel demand-fills a page on first access, so reserving a large
    /// span and committing a little costs only what is touched; fork skips it.
    pub backed: bool,
    /// Bytes Linux would give back after MADV_DONTNEED, not zero: a file's,
    /// an ELF segment's or a shared mapping's. This capsule cannot give them
    /// back, so it refuses that advice here.
    pub kept: bool,
}

impl Region {
    /// A span this capsule laid down, anonymous until a mark says otherwise.
    pub fn new(at: u64, len: u64, write: bool, exec: bool, backed: bool) -> Self {
        Self { at, len, write, exec, unproven: false, backed, kept: false }
    }
}
