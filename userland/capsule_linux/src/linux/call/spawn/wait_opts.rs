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

//! The options wait4 and waitid take, read as the ints they are and refused
//! as Linux's kernel_wait4 and kernel_waitid refuse them. Pure, so the host
//! proofs hold every refusal.

use crate::linux::abi::errno;

pub const WNOHANG: u64 = 1;
pub const WUNTRACED: u64 = 2;
pub const WSTOPPED: u64 = 2;
pub const WEXITED: u64 = 4;
pub const WCONTINUED: u64 = 8;
pub const WNOWAIT: u64 = 0x0100_0000;
pub const WNOTHREAD: u64 = 0x2000_0000;
pub const WALL: u64 = 0x4000_0000;
pub const WCLONE: u64 = 0x8000_0000;

const WAIT4: u64 = WNOHANG | WUNTRACED | WCONTINUED | WNOTHREAD | WALL | WCLONE;
const WAITID: u64 = WNOHANG | WEXITED | WSTOPPED | WCONTINUED | WNOWAIT | WNOTHREAD | WALL | WCLONE;

/// wait4's options, an int: EINVAL for one it does not know (WEXITED and
/// WNOWAIT among them, which are waitid's), else them with WEXITED, which
/// wait4 always means.
pub fn wait4_options(raw: u64) -> Result<u64, i64> {
    let options = u64::from(raw as u32);
    match options & !WAIT4 {
        0 => Ok(options | WEXITED),
        _ => Err(errno::EINVAL),
    }
}

/// waitid's options, an int: EINVAL for one it does not know, or for none
/// of WEXITED, WSTOPPED and WCONTINUED, which leaves nothing to wait for.
pub fn waitid_options(raw: u64) -> Result<u64, i64> {
    let options = u64::from(raw as u32);
    if options & !WAITID != 0 || options & (WEXITED | WSTOPPED | WCONTINUED) == 0 {
        return Err(errno::EINVAL);
    }
    Ok(options)
}
