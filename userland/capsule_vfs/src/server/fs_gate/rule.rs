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

//! The FileSystem decision, pure so the host proofs can walk every case.

/// Whether vfs serves a request from `sender_pid`.
///
/// The sender pid is stamped by the kernel and pid 0 is never handed to a
/// process, so it marks the kernel-internal client, which asks FileSystem of
/// the process it sends for before it sends (src/fs/vfs_capsule/capability.rs).
/// Any other sender is served only when `holds` says the kernel grants that
/// pid FileSystem. `holds` is asked once for each such request and its answer
/// is kept nowhere.
pub fn allows(sender_pid: u32, holds: impl FnOnce(u32) -> bool) -> bool {
    sender_pid == 0 || holds(sender_pid)
}
