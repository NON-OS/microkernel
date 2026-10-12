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

//! Who holds a claim, as pure rules `kernel_proofs` checks (L32).

/// A thread group of 0 means the thread stands for itself.
pub(crate) const fn group_of(tgid: u32, pid: u32) -> u32 {
    if tgid == 0 {
        pid
    } else {
        tgid
    }
}

/// Whether a thread of `group` other than `me` is still alive, given every
/// thread as `(pid, tgid, alive)`. While one is, the group keeps its devices.
pub(crate) fn others_alive(
    group: u32,
    me: u32,
    threads: impl IntoIterator<Item = (u32, u32, bool)>,
) -> bool {
    threads.into_iter().any(|(pid, tgid, alive)| pid != me && alive && group_of(tgid, pid) == group)
}
