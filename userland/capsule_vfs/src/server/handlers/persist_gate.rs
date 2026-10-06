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
//! Nothing persists unless the person asked for it at setup. The choice lives
//! in the policy service, which only setup and settings may write, and is
//! asked afresh on every request, so a machine that cannot answer is amnesic.

use nonos_libc::mk_debug;
use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

use crate::protocol::EACCES;

pub(super) fn may_persist() -> bool {
    lookup().and_then(|port| get_bool(port, Field::Persistent)) == Some(true)
}

/// Refuse a persist on an amnesic boot. Zeros are let through: they are
/// how a record is withdrawn, and removal must stay possible in either mode.
pub(super) fn require_persistent(data: &[u8]) -> Result<(), i32> {
    if may_persist() || data.iter().all(|b| *b == 0) {
        return Ok(());
    }
    let line = b"[VFS] refused persist: amnesic boot\n";
    let _ = mk_debug(line.as_ptr(), line.len());
    Err(EACCES)
}
