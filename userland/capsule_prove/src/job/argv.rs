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

/// argv, NUL separated, as the kernel hands it over.
fn args() -> Vec<Vec<u8>> {
    let needed = mk_args(core::ptr::null_mut(), 0);
    let Ok(len) = usize::try_from(needed) else {
        return Vec::new();
    };
    let mut buf = vec![0u8; len];
    let n = mk_args(buf.as_mut_ptr(), buf.len()).max(0) as usize;
    buf[..n.min(len)].split(|&b| b == 0).map(<[u8]>::to_vec).collect()
}

pub fn is_job() -> bool {
    args().iter().any(|a| a == JOB_ARG)
}

/// The op this job serves and the one pid it takes it from (bound.rs).
pub fn bound() -> Option<(u16, u32)> {
    super::bound::parse(&args())
}
