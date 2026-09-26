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

//! Consent to run what this machine builds and fetches.

use crate::syscall::{
    call_raw, N_MK_DEV_ROOT_CONFIRM, N_MK_DEV_ROOT_LOCAL, N_MK_LOCAL_CONSENT, N_MK_LOCAL_RESTORE,
};

/// Ask to enrol this machine's own build root, so what it installs can be
/// proved.
pub fn mk_dev_root_local() -> i64 {
    call_raw(N_MK_DEV_ROOT_LOCAL, [0, 0, 0, 0, 0, 0])
}

/// Complete the pending enrolment with the code the user read and typed.
pub fn mk_dev_root_confirm(code: u32) -> i64 {
    call_raw(N_MK_DEV_ROOT_CONFIRM, [code as u64, 0, 0, 0, 0, 0])
}

/// Let this machine run what it installs. `Ok(Some(token))` is consent that
/// lasts: keep the token and restore it on later boots. `Ok(None)` is consent
/// for this boot only, on a machine with no key to keep it with.
pub fn mk_local_consent_grant() -> Result<Option<[u8; 32]>, i64> {
    let mut token = [0u8; 32];
    match call_raw(N_MK_LOCAL_CONSENT, [0, token.as_mut_ptr() as u64, 0, 0, 0, 0]) {
        1 => Ok(Some(token)),
        0 => Ok(None),
        e => Err(e),
    }
}

/// Stop running what this machine installs.
pub fn mk_local_consent_revoke() -> i64 {
    call_raw(N_MK_LOCAL_CONSENT, [1, 0, 0, 0, 0, 0])
}

/// Restore consent from the token a grant returned on this machine.
pub fn mk_local_restore(token: &[u8; 32]) -> i64 {
    call_raw(N_MK_LOCAL_RESTORE, [token.as_ptr() as u64, 0, 0, 0, 0, 0])
}
