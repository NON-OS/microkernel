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

//! What glibc probes before main: how many CPUs, membarrier, clone3, getcpu.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/*
 * One CPU, always. The core count is a fingerprint (section 8: a guest must
 * not identify the machine), the same reason the fingerprint suite refuses
 * the host's pid count. A thread pool sized from this runs, only narrower.
 */
const CPUS: usize = 1;

pub fn sched_getaffinity(guest: &Guest, size: u64, mask: u64) -> u64 {
    let bytes = CPUS.div_ceil(64) * 8;
    if (size as usize) < bytes || size % 8 != 0 {
        return errno::fail(errno::EINVAL);
    }
    let mut out = [0u8; 8];
    out[0] = 1;
    match guest.write(mask, &out[..bytes]) == bytes as i64 {
        true => errno::ok(bytes as u64),
        false => errno::fail(errno::EFAULT),
    }
}

/// `getcpu`: the one CPU and node that affinity reports.
pub fn getcpu(guest: &Guest, cpu: u64, node: u64) -> u64 {
    for at in [cpu, node] {
        if at != 0 && guest.write(at, &0u32.to_le_bytes()) != 4 {
            return errno::fail(errno::EFAULT);
        }
    }
    errno::ok(0)
}

/// MEMBARRIER_CMD_QUERY answers that no command is offered, which glibc and
/// the runtimes that use it read as "fall back to their own barriers".
pub fn membarrier(cmd: u64) -> u64 {
    match cmd {
        0 => errno::ok(0),
        _ => errno::fail(errno::EINVAL),
    }
}

/// `clone3` is refused by name: glibc tries it first and falls back to
/// `clone`, which is served, on ENOSYS and only on ENOSYS.
pub fn clone3() -> u64 {
    errno::fail(errno::ENOSYS)
}
