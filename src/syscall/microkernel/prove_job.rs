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

//! `MkProveJob(kind)`: ask for the headless prover (plan/14, plan/15). Kind 1
//! PROVE: the caller owns app.store. Kind 2 DELIVER: the caller is at Vendor
//! authority and supervises a live family, so no capsule without one can
//! hold the prover for minutes. Anything else is EPERM. Init's loop spawns
//! the job, bound to the caller; the caller then sends its frame to
//! app.prove.job once that endpoint is registered.

use crate::process::foreign::seal::supervises_live;
use crate::security::attest_registry::attested_of;
use crate::security::dev_roots::Authority;
use crate::services::registry::caller_owns;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_PERM};
use crate::userspace::init::{prove_job_ask, ProveCaller};

const STORE: &str = "app.store";

/// 0 once a job is asked for; EBUSY while one is asked for, a prover runs,
/// or the supervisor's measurement delivered within the rate's span.
pub fn sys_prove_job(kind: u64) -> i64 {
    let Some(pid) = crate::process::current_pid() else {
        return ERRNO_PERM;
    };
    let vendor = attested_of(pid).filter(|c| c.authority == Authority::Vendor);
    let caller = ProveCaller {
        pid,
        owns_store: caller_owns(STORE),
        vendor: vendor.map(|c| c.measurement),
        supervises_live: supervises_live(pid),
    };
    let Some(ask) = prove_job_ask(kind, &caller) else {
        return ERRNO_PERM;
    };
    if crate::userspace::init::request_prove_job(ask) {
        0
    } else {
        ERRNO_BUSY
    }
}
