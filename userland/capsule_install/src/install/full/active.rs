/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Whether this installer owns the screen. It does on an install boot that
 * has no window manager: init starts none there until the installer has
 * ended. Launched from a desktop it is a window, as before.
 */

use core::mem::size_of;
use core::sync::atomic::{AtomicBool, Ordering};

use nonos_app_skeleton::clients::vfs;
use nonos_app_skeleton::discover::lookup_port;
use nonos_libc::procstat_header::BOOT_INSTALL_REQUESTED;
use nonos_libc::{mk_getpid, mk_proc_stat, ProcStatEntry, ProcStatHeader};
use nonos_policy_proto::setup_record::ANSWERS_PATH;

static FULL: AtomicBool = AtomicBool::new(false);
static SETUP_KEPT: AtomicBool = AtomicBool::new(false);

const LEN: usize = size_of::<ProcStatHeader>() + size_of::<ProcStatEntry>();

/* The boot menu's "Install NONOS" started this boot, and no desktop runs. */
pub fn wanted() -> bool {
    let mut buf = [0u8; LEN];
    if mk_proc_stat(buf.as_mut_ptr(), 1) < 0 {
        return false;
    }
    /* The buffer holds a whole header at its start, read unaligned. */
    let h = unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    h.boot_flags & BOOT_INSTALL_REQUESTED != 0 && lookup_port(b"wm").is_none()
}

pub fn active() -> bool {
    FULL.load(Ordering::Relaxed)
}

/* Setup left its answers in the vfs for the install to carry. */
pub fn setup_kept() -> bool {
    SETUP_KEPT.load(Ordering::Relaxed)
}

pub(super) fn set() {
    FULL.store(true, Ordering::Relaxed);
    let kept = vfs::stat(mk_getpid(), ANSWERS_PATH).is_ok_and(|(size, dir)| size > 0 && !dir);
    SETUP_KEPT.store(kept, Ordering::Relaxed);
}
