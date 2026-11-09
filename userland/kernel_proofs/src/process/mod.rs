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

//! Process control block pieces that are pure enough to drive on the host.

#[path = "../../../../src/process/core/inbox_name.rs"]
pub mod inbox_name;

// The per-process mmap VA allocator, and its guarded ranges.
#[allow(clippy::new_without_default)]
#[path = "../../../../src/process/mmap_va.rs"]
pub mod mmap_va;

#[cfg(test)]
mod mmap_va_tests;
// The rows one strided peer read moves, checked before any guest page.
#[path = "../../../../src/process/foreign/peer_rows_shape.rs"]
pub mod peer_rows_shape;
// The Wayland bridge's cut of a strided read into calls, held to the
// kernel's limits.
#[path = "../../../capsule_linux/src/linux/guest/mem_rows.rs"]
mod bridge_rows;
#[cfg(test)]
mod peer_rows_tests;

// The authority cells, epoch, mint and reissue of `process::caps`: what a
// grant, revoke or thread join writes and what the syscall gate reads.
#[cfg(test)]
pub mod caps;
