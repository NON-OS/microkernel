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

//! Who this driver serves: the kernel's block client, which arrives as pid
//! 0, and no one else. driver.usb_msc0 is the kernel's alone
//! (services/registry/held_table.rs), yet only the block ops checked their
//! sender: a capsule that reached this inbox by another name was served
//! every other op, and one sent configuration descriptor reset the bound
//! stick's residue quirk and failed every read after it. A health check
//! stays open, as on the other block drivers. Pure, so the host proofs hold
//! every op from every sender.

use crate::protocol::OP_HEALTHCHECK;

/// The kernel stamps every capsule's sender pid, and pid 0 is never handed
/// to a process, so it marks the kernel and nothing else.
pub fn admits(op: u16, sender_pid: u32) -> bool {
    op == OP_HEALTHCHECK || sender_pid == 0
}
