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

//! What a finished wait writes back: waitid's siginfo or wait4's status word,
//! and struct rusage. The kernel reports no CPU time for a guest, so rusage is
//! written as all zeros; Go always passes one, so it is still written, and the
//! log says once per family run that its zeros are not a measurement.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::linux::abi::errno;
use crate::linux::guest::siginfo::{SigInfo, CLD_EXITED, CLD_KILLED};
use crate::linux::guest::sigstate::SIGCHLD;
use crate::linux::guest::sigwaits::ChildWait;
use crate::linux::guest::Guest;

/// struct rusage: no CPU time is reported for a guest, so all of it is zero.
const RUSAGE_LEN: usize = 144;

/// Set once the zero rusage has been said in the log.
static RUSAGE_SAID: AtomicBool = AtomicBool::new(false);

/// Write what the caller asked for and give the value its call returns.
pub fn report(p: &Guest, w: &ChildWait, child: Option<(u32, i32)>, value: u64) -> u64 {
    let wrote = match (w.waitid, child) {
        (true, Some((pid, status))) => {
            let (code, st) = match status & 0x7f {
                0 => (CLD_EXITED, (status >> 8) & 0xff),
                s => (CLD_KILLED, s),
            };
            let info = SigInfo { value: st as u64, ..SigInfo::from(SIGCHLD, code, pid) };
            w.out == 0 || p.write(w.out, &info.bytes()) == 128
        }
        (true, None) => w.out == 0 || p.write(w.out, &[0u8; 128]) == 128,
        (false, Some((_, status))) => w.out == 0 || p.write(w.out, &status.to_le_bytes()) == 4,
        (false, None) => true,
    };
    let usage = child.is_none() || w.rusage == 0 || zero_rusage(p, w.rusage);
    match wrote && usage {
        true => value,
        false => errno::fail(errno::EFAULT),
    }
}

/// Write an all-zero rusage at `at`, saying so the first time in a family run.
fn zero_rusage(p: &Guest, at: u64) -> bool {
    if !RUSAGE_SAID.swap(true, Ordering::Relaxed) {
        let line = b"[LINUX] rusage: no CPU time is reported for a guest, written as zero\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    }
    p.write(at, &[0u8; RUSAGE_LEN]) > 0
}
