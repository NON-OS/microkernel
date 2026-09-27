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

//! Where a pid crosses between a guest and the kernel.
//!
//! Every call that takes a pid is rewritten before it is answered, and every
//! call that returns one is rewritten after, so no handler sees a guest's
//! number and no guest sees a kernel's.

use nonos_libc::ForeignFrame;

use crate::linux::abi::{errno, nr, nr_path as np};

use super::pid_ns::PidNs;

/// The frame with its pid arguments in kernel terms. A guest naming a process
/// outside its family is answered here, with the errno Linux would give.
pub fn frame_in(ns: &PidNs, frame: &ForeignFrame) -> Option<ForeignFrame> {
    let mut a = frame.args();
    if let Err(refused) = args_in(ns, frame.nr, &mut a) {
        let _ = nonos_libc::mk_foreign_reply(frame.pid, refused);
        return None;
    }
    let [arg0, arg1, arg2, arg3, arg4, arg5] = a;
    Some(ForeignFrame { arg0, arg1, arg2, arg3, arg4, arg5, ..*frame })
}

fn args_in(ns: &PidNs, call: u64, a: &mut [u64; 6]) -> Result<(), u64> {
    let (slots, missing): (&[usize], i64) = match call {
        nr::WAIT4 => (&[0], errno::ECHILD),
        np::KILL | np::TKILL | np::GETPGID | np::GETSID => (&[0], errno::ESRCH),
        np::SETPGID => (&[0, 1], errno::ESRCH),
        _ => return Ok(()),
    };
    for &i in slots {
        a[i] = one_in(ns, a[i]).ok_or(errno::fail(missing))?;
    }
    Ok(())
}

/// 0 and -1 mean the caller or everyone; a negative below that is a group,
/// named by its leader's pid.
fn one_in(ns: &PidNs, v: u64) -> Option<u64> {
    match v as i64 {
        -1..=0 => Some(v),
        g if g < 0 => {
            ns.inward(u32::try_from(g.checked_neg()?).ok()?).map(|k| -i64::from(k) as u64)
        }
        g => ns.inward(u32::try_from(g).ok()?).map(u64::from),
    }
}
