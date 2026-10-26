// NONOS Operating System (AGPL-3.0-or-later)

//! The restore driven a step a pass, each pass one vfs ask.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_getpid, mk_service_lookup, mk_uptime_ms};
use spin::Mutex;

use super::super::stage::{next, Got, Stage};
use super::ask::ask;
use crate::progress::{Progress, Say};

const RETRY_MS: u64 = 250;
const VFS_SERVICE: &[u8] = b"vfs_pool";

static FINISHED: AtomicBool = AtomicBool::new(false);
static NEXT_MS: AtomicU64 = AtomicU64::new(0);
static STATE: Mutex<(Stage, Progress)> = Mutex::new((Stage::Settle, Progress::new()));

pub fn tick() {
    let now = mk_uptime_ms() as u64;
    if FINISHED.load(Ordering::Relaxed) || now < NEXT_MS.load(Ordering::Relaxed) {
        return;
    }
    NEXT_MS.store(now.saturating_add(RETRY_MS), Ordering::Relaxed);
    let mut state = STATE.lock();
    let got = ask(state.0, mk_getpid());
    if let Say::FirstLate(pass) = state.1.pass(got == Got::Late) {
        crate::progress_say::first_late(b"restore", pass);
    }
    state.0 = next(state.0, got);
    if state.0 == Stage::Finished {
        // What was just put back is what the record already says: the
        // keeper writes only what changes from here on.
        crate::keep::written_now();
        FINISHED.store(true, Ordering::Relaxed);
        crate::progress_say::done(b"restore", state.1);
    }
}

/// Whether the store holds what an earlier boot kept, or nothing was kept:
/// until then the keeper must not write, or it would keep the defaults.
pub fn settled() -> bool {
    FINISHED.load(Ordering::Relaxed)
}

/// Whether vfs has loaded the store. Asked only once vfs has registered: a
/// call to a server that is not there yet fails at once, telling nothing.
pub(super) fn settle() -> Got {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(VFS_SERVICE.as_ptr(), VFS_SERVICE.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        return Got::No;
    }
    match vfs::store_settled() {
        Ok(true) => Got::Yes,
        Ok(false) => Got::No,
        Err(_) => Got::Late,
    }
}
