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

//! Who holds a claim and its grants: the caller's process, its thread group,
//! never the one thread that asked (L32). A driver may claim from one thread
//! and map, bind or wait from another, and a worker thread's exit leaves the
//! device with the process: only the group's last thread gives it back.

pub(crate) mod rule;

pub(crate) use rule::{group_of, others_alive};

/// The thread group `pid` belongs to, or `pid` itself when it has none (a
/// group of 0) or no longer exists.
pub fn holder_of(pid: u32) -> u32 {
    crate::process::with_process(pid, |p| group_of(p.thread_group_id(), pid)).unwrap_or(pid)
}
