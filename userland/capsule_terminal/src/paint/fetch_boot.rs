/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* The "os" row of the splash: NONOS and the boot profile chosen in the menu. */

use core::mem::size_of;

use nonos_libc::procstat_header::{
    BOOT_PROFILE_AIR_GAPPED, BOOT_PROFILE_HARDENED, BOOT_PROFILE_RECOVERY, BOOT_PROFILE_SAFE,
};
use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

const LEN: usize = size_of::<ProcStatHeader>() + size_of::<ProcStatEntry>();

pub fn os_line() -> &'static str {
    let mut buf = [0u8; LEN];
    if mk_proc_stat(buf.as_mut_ptr(), 1) < 0 {
        return "NONOS";
    }
    /* The buffer holds a whole header at its start, read unaligned. */
    let h = unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    let f = h.boot_flags;
    if f & BOOT_PROFILE_RECOVERY != 0 {
        "NONOS, Recovery boot: no network"
    } else if f & BOOT_PROFILE_SAFE != 0 {
        "NONOS, Safe Mode: no network, no audio"
    } else if f & BOOT_PROFILE_AIR_GAPPED != 0 {
        "NONOS, Air-Gapped: no network runs"
    } else if f & BOOT_PROFILE_HARDENED != 0 {
        "NONOS, Hardened boot"
    } else {
        "NONOS, Standard boot"
    }
}
