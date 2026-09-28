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

//! The sigevent timer_create reads: which signal a timer raises and with what
//! value, SIGEV_NONE for none, and SIGEV_THREAD_ID for one thread of the
//! process rather than the whole of it.

use crate::linux::abi::errno;
use crate::linux::guest::sigstate::NSIG;
use crate::linux::guest::sigtimer::PosixTimer;
use crate::linux::guest::Guest;

const SIGEV_SIGNAL: u32 = 0;
const SIGEV_NONE: u32 = 1;
const SIGEV_THREAD: u32 = 2;
const SIGEV_THREAD_ID: u32 = 4;

/// Read the sigevent at `sevp` into `t`, or the errno a bad one earns.
pub fn read_sigevent(guest: &Guest, sevp: u64, t: &mut PosixTimer) -> Result<(), u64> {
    let Some(raw) = guest.read(sevp, 20) else {
        return Err(errno::fail(errno::EFAULT));
    };
    let word = |i: usize| u32::from_le_bytes(raw[i..i + 4].try_into().unwrap_or([0; 4]));
    let (signo, notify) = (word(8), word(12));
    t.value = u64::from_le_bytes(raw[..8].try_into().unwrap_or([0; 8]));
    t.signo = if notify == SIGEV_NONE { 0 } else { signo as u8 };
    let bad_signo = notify != SIGEV_NONE && (signo == 0 || signo as usize > NSIG);
    match notify {
        _ if bad_signo => return Err(errno::fail(errno::EINVAL)),
        SIGEV_SIGNAL | SIGEV_NONE | SIGEV_THREAD => {}
        n if n == SIGEV_SIGNAL | SIGEV_THREAD_ID => {
            let tid = crate::linux::serve::kernel_pid(word(16));
            match tid.filter(|k| guest.live_threads().contains(k)) {
                Some(k) => t.tid = k,
                None => return Err(errno::fail(errno::EINVAL)),
            }
        }
        _ => return Err(errno::fail(errno::EINVAL)),
    }
    Ok(())
}
