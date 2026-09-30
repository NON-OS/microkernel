/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Whether this installer owns the screen. It does when no window manager
 * runs: init starts the installer ahead of the desktop when setup chose
 * Install or the boot menu asked for it, and starts no desktop until it has
 * ended. Launched from a desktop it is a window, as before.
 */

use core::sync::atomic::{AtomicBool, Ordering};

use nonos_app_skeleton::clients::vfs;
use nonos_app_skeleton::discover::lookup_port;
use nonos_libc::mk_getpid;
use nonos_policy_proto::setup_record::ANSWERS_PATH;

static FULL: AtomicBool = AtomicBool::new(false);
static SETUP_KEPT: AtomicBool = AtomicBool::new(false);

/* No desktop runs, so nothing but the installer is on screen. */
pub fn wanted() -> bool {
    lookup_port(b"wm").is_none()
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
