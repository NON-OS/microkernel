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

//! `MkProveJob()`: the store asks for the headless prover (plan/14). Only
//! the owner of app.store asks, under AppInstall; the market verifies and
//! never proves. The job is spawned by init's loop; the caller then sends its
//! PROVE to app.prove.job once that endpoint is registered.

use crate::services::registry::caller_owns;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_PERM};

const STORE: &str = "app.store";

/// 0 once a job is asked for; EBUSY while one is asked for or a prover runs,
/// as the job or as the window; EPERM for any caller but the store.
pub fn sys_prove_job() -> i64 {
    if !caller_owns(STORE) {
        return ERRNO_PERM;
    }
    if crate::userspace::init::request_prove_job() {
        0
    } else {
        ERRNO_BUSY
    }
}
