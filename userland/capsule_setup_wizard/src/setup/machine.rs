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
 * The machine as the kernel describes it to the Terminal and the process
 * manager: the header of a process listing that asks for one entry. Setup
 * reads this machine's memory and how this boot was started from it.
 */

use core::mem::size_of;

use nonos_libc::procstat_header::{
    BOOT_INSTALL_REQUESTED, BOOT_PROFILE_AIR_GAPPED, BOOT_PROFILE_RECOVERY, BOOT_PROFILE_SAFE,
};
use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

const LEN: usize = size_of::<ProcStatHeader>() + size_of::<ProcStatEntry>();

/* None when the kernel would not say. */
pub fn header() -> Option<ProcStatHeader> {
    let mut buf = [0u8; LEN];
    if mk_proc_stat(buf.as_mut_ptr(), 1) < 0 {
        return None;
    }
    /* The buffer holds a whole header at its start, read unaligned. */
    Some(unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) })
}

/* The boot menu's "Install NONOS" entry started this boot. */
pub fn install_boot() -> bool {
    header().is_some_and(|h| h.boot_flags & BOOT_INSTALL_REQUESTED != 0)
}

/* Air-Gapped, Safe Mode or Recovery: the kernel starts no network at all. */
pub fn network_off_boot() -> bool {
    let off = BOOT_PROFILE_AIR_GAPPED | BOOT_PROFILE_SAFE | BOOT_PROFILE_RECOVERY;
    header().is_some_and(|h| h.boot_flags & off != 0)
}

/* Safe Mode: no optional app starts, whatever the Apps step says. */
pub fn safe_boot() -> bool {
    header().is_some_and(|h| h.boot_flags & BOOT_PROFILE_SAFE != 0)
}
