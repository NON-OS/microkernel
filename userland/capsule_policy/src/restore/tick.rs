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

//! Asking vfs, between requests, whether setup kept anything.
//!
//! vfs loads the store from disk after this service has started, and until it
//! has, a missing file proves nothing. So the server loop calls in here between
//! requests, at most every quarter second, and it stops asking once vfs has
//! settled: the answers are restored, or there were none to restore.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_getpid, mk_service_lookup, mk_uptime_ms};
use nonos_policy_proto::setup_record::{check_record, is_done, ANSWERS_LEN, ANSWERS_PATH};
use nonos_policy_proto::setup_record::{DONE, DONE_PATH};

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
    FINISHED.store(true, Ordering::Relaxed);
    let pid = mk_getpid();
    let done = vfs::read_file(pid, DONE_PATH, DONE.len() as u32).is_ok_and(|raw| is_done(&raw));
    if !done {
        return;
    }
    let Ok(raw) = vfs::read_file(pid, ANSWERS_PATH, ANSWERS_LEN as u32) else {
        return super::apply::say(b"[POLICY] setup marker without answers: nothing restored\n");
    };
    match check_record(&raw) {
        Ok(record) => super::apply::apply(record),
        Err(why) => super::apply::refused(why.name()),
    }
}

fn vfs_registered() -> bool {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(VFS_SERVICE.as_ptr(), VFS_SERVICE.len(), &mut port, &mut pid);
    rc >= 0 && port != 0
}
