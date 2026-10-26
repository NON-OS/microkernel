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

//! Caller-side cap gate. The kernel-side virtio-blk client is
//! reachable by callers holding `CAP_DRIVER` or `CAP_STORAGE`, and by the
//! data volume's own I/O (block_device::volume_rule), as the other block
//! clients.

use super::error::DriverBlkError;

pub(super) fn gate_call() -> Result<u32, DriverBlkError> {
    let pid = match crate::process::current_pid() {
        Some(p) => p,
        None => return Err(DriverBlkError::NoCallerPid),
    };
    if !crate::hardware::block_device::disk_caller_admitted(pid) {
        return Err(DriverBlkError::AccessDenied);
    }
    Ok(pid)
}
