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

//! `MkDmaUnmap`: the holder gives one grant back.

use super::super::errnos::{ERRNO_INVAL, ERRNO_PERM};
use crate::hardware::broker::DmaError;
use crate::process::current_pid;

pub fn sys_dma_unmap(grant_id: u64) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    match crate::hardware::broker::dma_unmap_grant(pid, grant_id) {
        Ok(()) => 0,
        Err(DmaError::NotHolder) => ERRNO_PERM,
        Err(DmaError::UnknownGrant) => ERRNO_INVAL,
    }
}
