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

/*
 * The process a pid is a thread of. A worker thread maps DMA under
 * its own pid while its main thread registers the surface, so a grant and
 * a surface are matched by process, never by pid. The process table's read
 * lock nests inside the registry's locks; its writers take no registry lock.
 */
pub(super) fn group(pid: u32) -> u32 {
    crate::process::with_process(pid, |p| p.thread_group_id()).filter(|&g| g != 0).unwrap_or(pid)
}

/// Whether `a` and `b` are threads of one process.
pub(in crate::kernel_core::surface_registry) fn same_process(a: u32, b: u32) -> bool {
    a == b || group(a) == group(b)
}
