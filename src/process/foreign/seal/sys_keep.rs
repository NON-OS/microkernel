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
//! `MkSealPeek` and `MkSealDrop`, for a receipt (letter agent-02): the
//! prover copies the newest final record sealed for an app and keeps it in
//! place, writes the receipt over it, and only once the receipt is
//! delivered drops exactly that record. A failure anywhere between loses
//! nothing. The invariant is one record, one consume: MkSealRead's take or
//! MkSealDrop, never both, and never twice. Both are nonos.prove's alone,
//! by the gate MkSealRead has, and one nonos.prove runs at a time, the
//! window or one job: init's one loop spawns each, and refuses either while
//! the other lives (kernel_proofs prove_job/exclusion_tests.rs). A RECEIPT
//! job serves its one op on one thread, so no take runs between its peek
//! and its drop.

use super::table::SEALED;
use crate::syscall::microkernel::device_secret_caller;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_NOENT, ERRNO_NOMEM, ERRNO_PERM};

/// `MkSealPeek(app_id, buf, len)`: record | witness | list of the newest
/// record for `app_id`, as MkSealRead hands it, left sealed.
pub(crate) fn sys_seal_peek(app_id_ptr: u64, out_ptr: u64, out_len: u64) -> i64 {
    if !device_secret_caller() {
        return ERRNO_PERM;
    }
    let Some(app_id) = app_id_at(app_id_ptr) else { return ERRNO_FAULT };
    let bytes = {
        let sealed = SEALED.lock();
        let Some(s) = sealed.iter().rev().find(|s| s.app_id == app_id) else { return ERRNO_NOENT };
        if s.bytes_len() as u64 > out_len {
            return ERRNO_NOMEM;
        }
        s.bytes()
    };
    super::sys_read::write(out_ptr, bytes)
}

/// `MkSealDrop(app_id, seal_no)`: drop the one record sealed for `app_id`
/// as `seal_no`; ENOENT when there is none. Nothing is copied out.
pub(crate) fn sys_seal_drop(app_id_ptr: u64, seal_no: u64) -> i64 {
    if !device_secret_caller() {
        return ERRNO_PERM;
    }
    let Some(app_id) = app_id_at(app_id_ptr) else { return ERRNO_FAULT };
    let mut sealed = SEALED.lock();
    let at = sealed.iter().position(|s| s.app_id == app_id && s.seal_no == seal_no);
    /* Its witness cleared here, and again by Sealed's Drop (wipe.rs). */
    match at.and_then(|at| sealed.remove(at)) {
        Some(mut s) => {
            s.clear_witness();
            0
        }
        None => ERRNO_NOENT,
    }
}

fn app_id_at(ptr: u64) -> Option<[u8; 32]> {
    let mut app_id = [0u8; 32];
    crate::usercopy::copy_from_user(ptr, &mut app_id).ok().map(|_| app_id)
}
