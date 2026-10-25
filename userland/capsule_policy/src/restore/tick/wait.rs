// NONOS Operating System (AGPL-3.0-or-later)

//! Waiting for the store, then restoring once.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_getpid, mk_service_lookup, mk_uptime_ms};

use super::answers::restore;

const RETRY_MS: u64 = 250;
const VFS_SERVICE: &[u8] = b"vfs_pool";

static FINISHED: AtomicBool = AtomicBool::new(false);

static NEXT_MS: AtomicU64 = AtomicU64::new(0);

pub fn tick() {
    if FINISHED.load(Ordering::Relaxed) {
        return;
    }
    let now = mk_uptime_ms() as u64;
    if now < NEXT_MS.load(Ordering::Relaxed) {
        return;
    }
    NEXT_MS.store(now.saturating_add(RETRY_MS), Ordering::Relaxed);
    /*
     * Asked only once vfs has registered: a call to a server that is not
     * there yet would hold this loop, and every setting with it, until it
     * timed out.
     */
    if !vfs_registered() || vfs::store_settled() != Ok(true) {
        return;
    }
    let pid = mk_getpid();
    restore(pid);
    // What was just put back is what the record already says: the keeper
    // writes only what changes from here on.
    crate::keep::written_now();
    FINISHED.store(true, Ordering::Relaxed);
}

/// Whether the store holds what an earlier boot kept, or nothing was kept:
/// until then the keeper must not write, or it would keep the defaults.
pub fn settled() -> bool {
    FINISHED.load(Ordering::Relaxed)
}

fn vfs_registered() -> bool {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(VFS_SERVICE.as_ptr(), VFS_SERVICE.len(), &mut port, &mut pid);
    rc >= 0 && port != 0
}
