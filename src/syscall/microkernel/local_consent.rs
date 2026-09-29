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

//! `MkLocalConsent` and `MkLocalRestore`: the person's decision that this
//! machine runs what it installs, given once and kept as a token.

use crate::capabilities::caps_to_bits;
use crate::security::dev_roots::{grant_local_root, restore_local_root, revoke_local_root};
use crate::syscall::caps::current_caps_or_default;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};
use crate::usercopy::{copy_from_user, copy_to_user};

const GRANT: u64 = 0;
const REVOKE: u64 = 1;

/// `MkLocalConsent(op, token_ptr)`. A grant writes the 32-byte token and
/// returns 1, or returns 0 when this machine has no key to keep one with, in
/// which case the consent lasts this boot. A revoke returns 0.
pub fn sys_local_consent(op: u64, token_ptr: u64) -> i64 {
    let caps = caps_to_bits(&current_caps_or_default().permissions);
    match op {
        GRANT => match grant_local_root(caps) {
            Ok((_, Some(token))) => match copy_to_user(token_ptr, &token) {
                Ok(()) => 1,
                Err(_) => ERRNO_FAULT,
            },
            Ok((_, None)) => 0,
            Err(e) => e.to_errno(),
        },
        REVOKE => revoke_local_root(caps).map_or_else(|e| e.to_errno(), |()| 0),
        _ => ERRNO_INVAL,
    }
}

/// `MkLocalRestore(token_ptr)`. Re-enrols the local root from a token a grant
/// returned on this machine. It cannot grant anything: a token nobody was
/// given does not verify.
pub fn sys_local_restore(token_ptr: u64) -> i64 {
    let mut token = [0u8; 32];
    if copy_from_user(token_ptr, &mut token).is_err() {
        return ERRNO_FAULT;
    }
    restore_local_root(&token).map_or_else(|e| e.to_errno(), |_| 0)
}
