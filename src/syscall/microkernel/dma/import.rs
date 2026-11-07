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

//! `MkDmaImportSurface` / `MkDmaUnimportSurface` handlers (broker
//! dma/import). Cap-gated by `Dma` and `GraphicsSurfaceMap` at the contract
//! layer. The import writes its answer back, and is taken out again if that
//! copy fails.

use core::mem::size_of;

use super::super::errnos::{
    ERRNO_FAULT, ERRNO_INVAL, ERRNO_NODEV, ERRNO_NOMEM, ERRNO_NOSPC, ERRNO_PERM, ERRNO_STALE,
};
use crate::hardware::broker::{DmaImportError, DmaUnimportError};
use crate::process::current_pid;
use crate::usercopy::{validate_user_write, write_user_value};

#[repr(C)]
#[derive(Clone, Copy)]
struct DmaImportOut {
    device_addr: u64,
    length: u64,
    import_id: u64,
}

const _: () = assert!(size_of::<DmaImportOut>() == 24);

pub fn sys_dma_import_surface(device_id: u64, claim_epoch: u64, handle: u64, out_ptr: u64) -> i64 {
    let Some(pid) = current_pid() else { return ERRNO_PERM };
    let pid = crate::hardware::broker::holder_of(pid);
    if out_ptr == 0 || validate_user_write(out_ptr, size_of::<DmaImportOut>()).is_err() {
        return ERRNO_FAULT;
    }
    let r = match crate::hardware::broker::dma_import_surface(pid, device_id, claim_epoch, handle) {
        Ok(r) => r,
        Err(e) => return errno_for(e),
    };
    let out = DmaImportOut { device_addr: r.device_addr, length: r.length, import_id: r.import_id };
    if write_user_value(out_ptr, &out).is_err() {
        let _ = crate::hardware::broker::dma_unimport_surface(pid, r.import_id);
        return ERRNO_FAULT;
    }
    0
}

pub fn sys_dma_unimport_surface(import_id: u64) -> i64 {
    let Some(pid) = current_pid() else { return ERRNO_PERM };
    let pid = crate::hardware::broker::holder_of(pid);
    match crate::hardware::broker::dma_unimport_surface(pid, import_id) {
        Ok(()) => 0,
        Err(DmaUnimportError::NotHolder) => ERRNO_PERM,
        Err(DmaUnimportError::Unknown) => ERRNO_INVAL,
    }
}

fn errno_for(e: DmaImportError) -> i64 {
    match e {
        DmaImportError::NotClaimed | DmaImportError::NotAttached => ERRNO_PERM,
        DmaImportError::StaleEpoch => ERRNO_STALE,
        // No unit confines the device: the caller composes instead.
        DmaImportError::Unconfined => ERRNO_NODEV,
        DmaImportError::BadHandle => ERRNO_INVAL,
        DmaImportError::TooMany => ERRNO_NOSPC,
        DmaImportError::Failed => ERRNO_NOMEM,
    }
}
