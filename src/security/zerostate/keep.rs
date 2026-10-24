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

//! The kernel's half of "nothing persists unless the person asked".
//!
//! The file service already refuses to keep a file on a boot that keeps
//! nothing, but any holder of StoreWrite can call the store write directly.
//! This check sits in that call, so the promise no longer rests on one
//! capsule behaving. The disk plan is read afresh on every write: there is
//! no flag to set, and nothing a capsule can say turns keeping on.

use super::keep_rule::{disk_of, may_write, Disk};

/// What the selected disk's plan says, read now.
pub(crate) fn disk() -> Disk {
    disk_of(crate::fs::blockfs_volume::plan_keeps_volume())
}

/// True when this write may reach the store. A refusal is said on serial
/// with the disk's reading, so a boot test can find it.
pub(crate) fn store_write_allowed(bytes: &[u8]) -> bool {
    let disk = disk();
    if may_write(disk, bytes) {
        return true;
    }
    crate::sys::serial::print(match disk {
        Disk::Unread => b"[ZS] store write refused: the disk plan could not be read\n",
        _ => b"[ZS] store write refused: this boot keeps nothing\n",
    });
    false
}
