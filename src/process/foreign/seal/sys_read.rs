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

//! `MkSealRead(app_id, buf, len, pid)`: nonos.prove takes the newest final
//! record sealed for an app, as record | witness | measurement list. The
//! record is dropped as it is read, so one sealed run yields at most one
//! proof. With app_id 0 it is the live form (plan/15): the family `pid`
//! supervises, as it stands, for DELIVER; nothing is dropped.

use alloc::vec::Vec;

use super::sealed::Sealed;
use super::table::SEALED;
use crate::syscall::microkernel::device_secret_caller;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_NOENT, ERRNO_NOMEM, ERRNO_PERM};

fn take(app_id: &[u8; 32], room: u64) -> Result<Sealed, i64> {
    let mut sealed = SEALED.lock();
    let at = sealed.iter().rposition(|s| s.app_id == *app_id).ok_or(ERRNO_NOENT)?;
    if sealed[at].bytes_len() as u64 > room {
        return Err(ERRNO_NOMEM);
    }
    sealed.remove(at).ok_or(ERRNO_NOENT)
}

pub(crate) fn sys_seal_read(app_id_ptr: u64, out_ptr: u64, out_len: u64, pid: u64) -> i64 {
    if !device_secret_caller() {
        return ERRNO_PERM;
    }
    if app_id_ptr == 0 {
        return match super::live::snapshot(pid, out_len) {
            Ok(s) => write(out_ptr, s.bytes()),
            Err(e) => e,
        };
    }
    let mut app_id = [0u8; 32];
    if crate::usercopy::copy_from_user(app_id_ptr, &mut app_id).is_err() {
        return ERRNO_FAULT;
    }
    let s = match take(&app_id, out_len) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let n = write(out_ptr, s.bytes());
    if n == ERRNO_FAULT {
        // A bad buffer loses nothing: the record goes back for the next read.
        SEALED.lock().push_back(s);
    }
    n
}

/// The copy holds the witness: cleared whether or not the write went through.
fn write(out_ptr: u64, mut bytes: Vec<u8>) -> i64 {
    let n = match crate::usercopy::write_user_bytes(out_ptr, &bytes) {
        Ok(()) => i64::try_from(bytes.len()).unwrap_or(ERRNO_FAULT),
        Err(_) => ERRNO_FAULT,
    };
    super::wipe::wipe_bytes(&mut bytes);
    n
}
