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

//! Whether a capsule may claim this name on this port.

use crate::services::registry::{
    caller_has_register_right, is_reserved_service, is_runtime_registrable, lookup_service,
};
use crate::syscall::microkernel::errnos::ERRNO_PERM;

pub(super) fn allowed(name: &str, port: u32, pid: u32) -> Result<(), i64> {
    if name.starts_with("proc.") || name.starts_with("endpoint.") {
        return Err(ERRNO_PERM);
    }
    // Core service names and ports belong to the kernel spawn path.
    if is_reserved_service(name, port) {
        return Err(ERRNO_PERM);
    }
    if lookup_service(name).is_some_and(|e| e.pid == pid && e.port == port) {
        return Ok(());
    }
    // The right alone was not enough.
    match caller_has_register_right() && is_runtime_registrable(name) {
        true => Ok(()),
        false => Err(ERRNO_PERM),
    }
}
