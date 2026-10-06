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
//! `MkDataRead` into a guest: when its sixth argument names a guest the
//! caller supervises, `buf` is an address in that guest. The range is
//! opened a stage at a time into a kernel stage and written from there
//! straight into the guest's pages, so a model read crosses to the guest
//! once and never lands in the supervisor's memory. Every sector is
//! verified before a byte of it reaches the stage, and the stage is wiped
//! before it is freed.

use alloc::vec::Vec;

use super::super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};
use super::errno::errno;

/// One stage: a device run of sealed sectors opens into about this much.
const STAGE: usize = 32 << 10;
/// The first address of the kernel half.
const USER_VA_END: u64 = 0x0000_8000_0000_0000;

pub(super) fn read_into_guest(name: &[u8], offset: u64, peer: u64, addr: u64, len: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let supervisor = u32::try_from(peer).ok().and_then(crate::process::foreign::supervisor_of);
    if supervisor != Some(caller) {
        return ERRNO_PERM;
    }
    if addr.checked_add(len).is_none_or(|end| end > USER_VA_END) {
        return ERRNO_FAULT;
    }
    let mut stage = Vec::new();
    if stage.try_reserve_exact(STAGE).is_err() {
        crate::log::warn!("[DATA] guest read refused: no {} bytes of heap to stage it", STAGE);
        return ERRNO_NOMEM;
    }
    stage.resize(STAGE, 0u8);
    let mut refused = 0i64;
    let got =
        crate::fs::blockfs_volume::read_to(name, offset, len as usize, &mut stage, |at, b| {
            match crate::process::foreign::fill_guest(caller, peer, addr + at as u64, b) {
                Ok(()) => true,
                Err(e) => {
                    refused = e;
                    false
                }
            }
        });
    crate::crypto::constant_time::secure_zero(&mut stage);
    match got {
        Err(e) => errno(e),
        /*
         * Bytes already in the guest are counted, as a short read.
         */
        Ok(0) if refused != 0 => refused,
        Ok(n) => n as i64,
    }
}
