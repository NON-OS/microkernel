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

//! Who this driver serves. Raw sectors hold every partition on the disk,
//! another system's unencrypted files among them, and the store the next boot
//! trusts, so only the kernel's own client and a sender the kernel says holds
//! StoreWrite (the installer, vfs) reach the medium. A health check stays
//! open. Pure, so the decision is held on the host.

use crate::protocol::OP_HEALTHCHECK;

/// The kernel stamps every capsule's sender pid, and pid 0 is never handed to
/// a process, so it marks the kernel-internal client and nothing else.
pub fn allows(op: u16, sender_pid: u32, holds_store_write: bool) -> bool {
    op == OP_HEALTHCHECK || sender_pid == 0 || holds_store_write
}
