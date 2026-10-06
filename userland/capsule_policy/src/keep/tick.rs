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

//! Writing the settings record between requests, once changes are quiet.

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_getpid, mk_uptime_ms};
use nonos_policy_proto::settings_record::{SETTINGS_DIR, VALUES_PATH};
use nonos_policy_proto::Field;

use crate::store::get_bool;
use crate::store::state::CHANGES;

/// How long the store must be still before the record is written, so a
/// slider dragged across its range is one write, not one per step.
const QUIET_MS: u64 = 1_000;
/// How long a write that failed waits before the next try.
const RETRY_MS: u64 = 5_000;

/// The change count the record on disk holds.
static WRITTEN: AtomicU32 = AtomicU32::new(0);
/// The change count last seen, and when it was first seen.
static SEEN: AtomicU32 = AtomicU32::new(0);
static SEEN_AT: AtomicU64 = AtomicU64::new(0);
static NOT_BEFORE: AtomicU64 = AtomicU64::new(0);

/// The store as it stands is what the record holds.
pub fn written_now() {
    let c = CHANGES.load(Ordering::Relaxed);
    WRITTEN.store(c, Ordering::Relaxed);
    SEEN.store(c, Ordering::Relaxed);
}

pub fn tick() {
    if !crate::restore::settled() {
        return;
    }
    let c = CHANGES.load(Ordering::Relaxed);
    if c == WRITTEN.load(Ordering::Relaxed) {
        return;
    }
    let now = mk_uptime_ms().max(0) as u64;
    if c != SEEN.swap(c, Ordering::Relaxed) {
        SEEN_AT.store(now, Ordering::Relaxed);
        return;
    }
    let quiet = now >= SEEN_AT.load(Ordering::Relaxed).saturating_add(QUIET_MS);
    if !quiet || now < NOT_BEFORE.load(Ordering::Relaxed) {
        return;
    }
    // An amnesic boot keeps nothing; the count is taken as written, so
    // turning Persistent on later writes on the next change.
    if get_bool::get(Field::Persistent) != Some(true) {
        WRITTEN.store(c, Ordering::Relaxed);
        return;
    }
    match write() {
        Ok(()) => WRITTEN.store(c, Ordering::Relaxed),
        Err(_) => NOT_BEFORE.store(now.saturating_add(RETRY_MS), Ordering::Relaxed),
    }
}

/// The record, written afresh and kept: a kept file is replaced only by one
/// of the same length, and one an earlier boot kept belongs to nobody, so it
/// is unlinked first, as setup does with its answers.
fn write() -> Result<(), &'static str> {
    let pid = mk_getpid();
    let bytes = super::snapshot::record();
    let _ = vfs::mkdir(pid, b"/nonos");
    let _ = vfs::mkdir(pid, SETTINGS_DIR);
    let _ = vfs::unlink(pid, VALUES_PATH);
    vfs::write_file(pid, VALUES_PATH, &bytes)?;
    vfs::persist(pid, VALUES_PATH)
}
