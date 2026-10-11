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

//! Whether this instance is the headless job: init spawns it with
//! "--nonos-prove-job" (src/userspace/capsule_prove/job.rs).

use alloc::vec;

use nonos_libc::mk_args;

const JOB_ARG: &[u8] = b"--nonos-prove-job";

pub fn is_job() -> bool {
    let needed = mk_args(core::ptr::null_mut(), 0);
    let Ok(len) = usize::try_from(needed) else {
        return false;
    };
    let mut buf = vec![0u8; len];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    let Ok(n) = usize::try_from(n) else {
        return false;
    };
    // argv arrives NUL separated; the marker is one whole argument.
    buf[..n.min(len)].split(|&b| b == 0).any(|arg| arg == JOB_ARG)
}
