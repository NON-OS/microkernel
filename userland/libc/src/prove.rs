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

//! The store's ask for the headless prover (plan/14).

use crate::syscall::{call_raw, N_MK_CALLER_MEAS, N_MK_PROVE_JOB};

/// Ask init for nonos.prove on app.prove.job: 0, or EBUSY while one is asked
/// for or a prover runs, or EPERM for any caller but app.store. The PROVE
/// itself goes to app.prove.job in nonos_proof_abi::wire's frames.
pub fn mk_prove_job() -> i64 {
    call_raw(N_MK_PROVE_JOB, [0; 6])
}

/// The measurement and authority byte the kernel recorded for `pid`, the
/// sender of a message: 0 with `out` written, ESRCH once it exited, EPERM for
/// any caller but nonos.prove.
pub fn mk_caller_meas(pid: u32, out: &mut [u8; 33]) -> i64 {
    call_raw(N_MK_CALLER_MEAS, [u64::from(pid), out.as_mut_ptr() as u64, 33, 0, 0, 0])
}
