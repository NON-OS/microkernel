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

//! MMIO ordering on riscv64, from the RISC-V memory model's FENCE: its
//! predecessor and successor sets name device input (i), device output (o),
//! memory reads (r) and memory writes (w), so each barrier says exactly which
//! accesses it orders against which.

pub(super) mod access;
pub(super) mod fences;

pub(crate) use access::{read_acquire, read_relaxed, write_relaxed, write_release};
pub(crate) use fences::{fence_full, fence_reads, fence_writes};
