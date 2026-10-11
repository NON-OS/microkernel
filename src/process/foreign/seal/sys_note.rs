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

//! `MkSealNote(slot, ptr)`: a vendor supervisor's two words in its seal.

use super::table::with_family;
use super::vendor::vendor_caller;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT};

const APP_ID: u64 = 0;
const DIAG: u64 = 1;

/// Slot 0 is the app, set once and before any call is absorbed, so a record
/// never names an app for calls made before it was named. Slot 1 is the
/// supervisor's own diagnostic head, private, the last write kept.
pub(crate) fn sys_seal_note(slot: u64, ptr: u64) -> i64 {
    let caller = match vendor_caller() {
        Ok(c) => c,
        Err(e) => return e,
    };
    if slot != APP_ID && slot != DIAG {
        return ERRNO_INVAL;
    }
    let mut word = [0u8; 32];
    if crate::usercopy::copy_from_user(ptr, &mut word).is_err() {
        return ERRNO_FAULT;
    }
    let noted = with_family(caller, |f| {
        if slot == DIAG {
            f.diag = word;
            0
        } else if !f.started && f.app_id.is_none() {
            f.app_id = Some(word);
            0
        } else {
            ERRNO_BUSY
        }
    });
    noted.unwrap_or(ERRNO_NOENT)
}
