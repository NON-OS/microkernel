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

//! `MkServeOne`: a Linux run serves its guest's one listener to one capsule
//! (SERVE_ONE).
//!
//! The personality calls it at the guest's first listen, naming the capsule
//! its policy serves and the listing key it bound at start (app/bind.rs).
//! The kernel registers `linux.serve.<capsule>` for the calling process,
//! held to that capsule, so only the process owning the capsule's endpoint
//! reaches it, by name, port or pid (services/registry/held_gate.rs). Only
//! the process registered as app.linux.run or app.linux.run.net may ask,
//! which holds ForeignExec (the cap table's gate), and only for a pair SERVES
//! lists (registry/serves.rs, serve_rule.rs).

use crate::services::registry::{admit_serve, caller_group, register_served, RegError};
use crate::services::registry::{ServeRefusal, SERVE_NAME_MAX};
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL};
use crate::syscall::microkernel::errnos::{ERRNO_NOMEM, ERRNO_PERM};
use crate::userspace::capsule_linux::is_run_role;

const KEY_LEN: usize = 32;

/// `MkServeOne(name_ptr, name_len, key_ptr, key_len)`. The endpoint's port;
/// EINVAL for a bad name or a key that is not 32 bytes; EFAULT for one the
/// kernel cannot read; EPERM unless the caller is the run role and SERVES
/// pairs the capsule with the key; EBUSY when the capsule is served, or the
/// caller serves one already; ENOMEM when the registry is full.
pub fn sys_serve_one(name_ptr: u64, name_len: usize, key_ptr: u64, key_len: usize) -> i64 {
    if name_len == 0 || name_len > SERVE_NAME_MAX || key_len != KEY_LEN {
        return ERRNO_INVAL;
    }
    let (mut name, mut key) = ([0u8; SERVE_NAME_MAX], [0u8; KEY_LEN]);
    let name = &mut name[..name_len];
    let copy = |ptr, dst: &mut [u8]| crate::usercopy::copy_from_user(ptr, dst).is_ok();
    if !copy(name_ptr, name) || !copy(key_ptr, &mut key) {
        return ERRNO_FAULT;
    }
    let Some(caller) = caller_group() else { return ERRNO_PERM };
    let entry = match admit_serve(name, &key, is_run_role(caller)) {
        Ok(entry) => entry,
        Err(ServeRefusal::BadName) => return ERRNO_INVAL,
        Err(ServeRefusal::NotTheRun | ServeRefusal::NotPaired) => return ERRNO_PERM,
    };
    match register_served(entry, caller) {
        Ok(port) => i64::from(port),
        Err(RegError::Exists) => ERRNO_BUSY,
        Err(RegError::Full) => ERRNO_NOMEM,
        Err(RegError::PermissionDenied | RegError::NotFound) => ERRNO_PERM,
    }
}
