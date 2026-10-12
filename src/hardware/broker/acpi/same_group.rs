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

//! A thread is its process: a driver claims a device from one thread and
//! asks for its D-state from another, and both are the holder. So holders
//! are compared by thread group, as the registry compares service owners
//! (`services/registry/peers_check.rs`): a pid whose group reads as 0, or
//! that names no process, stands for itself. Pure: `kernel_proofs`
//! mounts it with a model of the process table.

/// The thread group `pid` belongs to, by the table `group_of` reads.
pub fn group(pid: u32, group_of: impl Fn(u32) -> Option<u32>) -> u32 {
    group_of(pid).filter(|g| *g != 0).unwrap_or(pid)
}

/// Whether `asker` is the holder `holder` is, thread or leader alike.
pub fn same_holder(holder: u32, asker: u32, group_of: impl Fn(u32) -> Option<u32> + Copy) -> bool {
    asker != 0 && group(holder, group_of) == group(asker, group_of)
}
