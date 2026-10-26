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

use super::error::DriverNvmeError;

pub(super) fn gate_call() -> Result<u32, DriverNvmeError> {
    let pid = match crate::process::current_pid() {
        Some(p) => p,
        None => return Err(DriverNvmeError::NoCallerPid),
    };
    /*
     * The same authority as the virtio-blk client: a driver, or the holder of
     * the store (vfs persisting packages), or the data volume's own I/O
     * (block_device::volume_rule). Without
     * the second, a machine whose only disk sat behind this controller
     * refused every store write from vfs and every data volume read.
     */
    if !crate::hardware::block_device::disk_caller_admitted(pid) {
        return Err(DriverNvmeError::AccessDenied);
    }
    Ok(pid)
}
