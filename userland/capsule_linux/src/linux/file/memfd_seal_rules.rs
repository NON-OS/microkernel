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

/* A memfd's seals as mm/memfd.c adds and checks them. Pure, so the host proofs hold it. */

use crate::linux::abi::errno;

pub const F_ADD_SEALS: u64 = 1033;
pub const F_GET_SEALS: u64 = 1034;
pub const SEAL: u32 = 0x1;
pub const SHRINK: u32 = 0x2;
pub const GROW: u32 = 0x4;
pub const WRITE: u32 = 0x8;
pub const FUTURE_WRITE: u32 = 0x10;
pub const EXEC: u32 = 0x20;
const ALL: u32 = 0x3f;
const MFD_ALLOW_SEALING: u64 = 0x2;
const MFD_NOEXEC_SEAL: u64 = 0x8;

/* Without MFD_ALLOW_SEALING a memfd starts with SEAL; MFD_NOEXEC_SEAL starts it with EXEC. */
pub fn first_seals(flags: u64) -> u32 {
    match (flags & MFD_NOEXEC_SEAL != 0, flags & MFD_ALLOW_SEALING != 0) {
        (true, _) => EXEC,
        (false, true) => 0,
        (false, false) => SEAL,
    }
}

/*
 * The seals after F_ADD_SEALS asks for `want`: an unknown bit is EINVAL,
 * anything once SEAL is set EPERM, and WRITE while a writable shared map
 * exists EBUSY. EXEC on a memfd that could still be run seals writes and
 * the size as well, as Linux does when the mode allows exec.
 */
pub fn add(now: u32, want: u64, writably_mapped: bool) -> Result<u32, i64> {
    let mut want = want as u32;
    if want & !ALL != 0 {
        return Err(errno::EINVAL);
    }
    if now & SEAL != 0 {
        return Err(errno::EPERM);
    }
    if want & EXEC != 0 && now & EXEC == 0 {
        want |= SHRINK | GROW | WRITE | FUTURE_WRITE;
    }
    if want & WRITE != 0 && now & WRITE == 0 && writably_mapped {
        return Err(errno::EBUSY);
    }
    Ok(now | want)
}

/* Whether a resize from `size` to `to` is refused: SHRINK down, GROW up. */
pub fn resize_refused(seals: u32, size: u64, to: u64) -> bool {
    (to < size && seals & SHRINK != 0) || (to > size && seals & GROW != 0)
}

/* WRITE and FUTURE_WRITE refuse a write, and a new writable shared map. */
pub fn writes_refused(seals: u32) -> bool {
    seals & (WRITE | FUTURE_WRITE) != 0
}
