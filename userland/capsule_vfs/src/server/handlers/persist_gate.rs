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

//! Whether anything may reach the disk this boot.
//!
//! Nothing persists unless the person asked for it at setup. The kernel
//! holds that choice once the policy service has told it, and its store write
//! refuses everything else, so vfs asks the kernel, afresh on every request.
//!
//! Not the policy service: its keeper writes the settings record through vfs
//! and waits for the reply, so a call back from here would wait on a server
//! that is waiting on this one, and each write would fail at the deadline.

use nonos_libc::{mk_debug, mk_keeping};

use super::persist_rule::{keep, Keep};
use crate::protocol::EACCES;

pub(super) fn may_persist() -> bool {
    mk_keeping()
}

/// Refuse a persist on an amnesic boot. Zeros are let through as a
/// withdrawal, which the disk side holds to an existing record of the same
/// length: removal stays possible in either mode, and an amnesic boot never
/// adds a name, length or digest to the disk.
pub(super) fn require_persistent(data: &[u8]) -> Result<Keep, i32> {
    if let Some(k) = keep(may_persist(), data) {
        return Ok(k);
    }
    let line = b"[VFS] refused persist: amnesic boot\n";
    let _ = mk_debug(line.as_ptr(), line.len());
    Err(EACCES)
}
