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

//! Whether this instance is the headless job, and whom it serves: init
//! spawns it with "--nonos-prove-job", then "prove" or "deliver", then the
//! pid that asked (src/userspace/capsule_prove/job_argv.rs).

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_args;

const JOB_ARG: &[u8] = b"--nonos-prove-job";
/// More argv than this is not init's three words: no job is bound.
const ARGS_MAX: usize = 64 * 1024;

/// argv, NUL separated, as the kernel hands it over.
fn args() -> Vec<Vec<u8>> {
    let needed = mk_args(core::ptr::null_mut(), 0);
    let Some(len) = usize::try_from(needed).ok().filter(|&n| n <= ARGS_MAX) else {
        return Vec::new();
    };
    let mut buf = vec![0u8; len];
    let n = usize::try_from(mk_args(buf.as_mut_ptr(), buf.len())).unwrap_or(0);
    buf[..n.min(len)].split(|&b| b == 0).map(<[u8]>::to_vec).collect()
}

pub fn is_job() -> bool {
    args().iter().any(|a| a == JOB_ARG)
}

/// The op this job serves and the one pid it takes it from (bound.rs).
pub fn bound() -> Option<(u16, u32)> {
    super::bound::parse(&args())
}

/// Whether init spawned this job for a receipt, read into the stack before any
/// allocation, so a receipt job never holds the prover's floor. Init's argv
/// is three short words; a longer one is no receipt job's.
pub fn is_receipt() -> bool {
    let mut buf = [0u8; 128];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    usize::try_from(n)
        .ok()
        .filter(|&n| n <= buf.len())
        .is_some_and(|n| super::bound::receipt(&buf[..n]))
}
