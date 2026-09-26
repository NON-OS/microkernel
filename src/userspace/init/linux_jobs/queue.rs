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

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

/// Deeper than a person clicks, shallower than a caller in a loop can grow.
const DEPTH: usize = 8;

#[derive(PartialEq, Eq)]
pub(super) enum Job {
    /// A listing and the release asked for, which is empty for the default.
    Install(String, String),
    /// A package whose program should start.
    Run(String),
}

static PENDING: Mutex<Vec<Job>> = Mutex::new(Vec::new());

/// Queue an install. False when full or already queued, which the caller
/// reports as busy.
pub(crate) fn request_install(listing: String, release: String) -> bool {
    push(Job::Install(listing, release))
}

/// Queue a run. False when full or already queued.
pub(crate) fn request_run(package: String) -> bool {
    push(Job::Run(package))
}

fn push(job: Job) -> bool {
    let mut q = PENDING.lock();
    if q.len() >= DEPTH || q.contains(&job) {
        return false;
    }
    q.push(job);
    drop(q);
    super::super::instance_spawn::raise_drain();
    true
}

/// Whether a job is waiting; a contended lock is a push in flight.
pub(crate) fn has_pending() -> bool {
    PENDING.try_lock().map_or(true, |q| !q.is_empty())
}

pub(super) fn take() -> Vec<Job> {
    core::mem::take(&mut *PENDING.lock())
}
