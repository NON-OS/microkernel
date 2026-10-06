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

/*
 * The flags of the calls that make a descriptor of a kind of their own,
 * checked as Linux checks them. Pure, so the host proofs hold it.
 */

use crate::linux::abi::errno;

/* EPOLL_CLOEXEC is O_CLOEXEC (include/uapi/linux/eventpoll.h). */
const EPOLL_CLOEXEC: u64 = 0o2000000;

/* memfd_create's flags and huge page size field (include/uapi/linux/memfd.h). */
const MFD_CLOEXEC: u64 = 0x1;
const MFD_HUGETLB: u64 = 0x4;
const MFD_NOEXEC_SEAL: u64 = 0x8;
const MFD_EXEC: u64 = 0x10;
const MFD_ALL: u64 = 0x1f;
const MFD_HUGE_SIZES: u64 = 0x3f << 26;

/*
 * Whether the list is close-on-exec. The flags are an int, and any but
 * EPOLL_CLOEXEC is EINVAL, as do_epoll_create (fs/eventpoll.c) refuses it.
 */
pub fn epoll_flags(flags: u64) -> Result<bool, i64> {
    let flags = u64::from(flags as u32);
    if flags & !EPOLL_CLOEXEC != 0 {
        return Err(errno::EINVAL);
    }
    Ok(flags != 0)
}

/*
 * Whether the memfd is close-on-exec, as mm/memfd.c refuses its flags: an
 * unsigned int, nothing outside the five flags, a huge page size only with
 * MFD_HUGETLB, and never MFD_EXEC with MFD_NOEXEC_SEAL. The rest change
 * nothing here: a memfd is ordinary memory either way.
 */
pub fn memfd_flags(flags: u64) -> Result<bool, i64> {
    let flags = u64::from(flags as u32);
    let known = match flags & MFD_HUGETLB {
        0 => MFD_ALL,
        _ => MFD_ALL | MFD_HUGE_SIZES,
    };
    let both = MFD_EXEC | MFD_NOEXEC_SEAL;
    if flags & !known != 0 || flags & both == both {
        return Err(errno::EINVAL);
    }
    Ok(flags & MFD_CLOEXEC != 0)
}
