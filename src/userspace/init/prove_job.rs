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

//! The prove-job queue: MkProveJob asks, init's loop spawns the
//! headless prover. One job at a time, so the queue is one flag; the spawn
//! runs in init's context, as every instance spawn does, never in the caller's
//! syscall.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::userspace::capsule_prove::{prover_live, spawn_prove_job};

static PENDING: AtomicBool = AtomicBool::new(false);

/// Ask for a job. False when one is already asked for or a prover is live,
/// as the job or as the window.
pub(crate) fn request() -> bool {
    !prover_live()
        && PENDING.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok()
}

pub(crate) fn has_pending() -> bool {
    PENDING.load(Ordering::Acquire)
}

pub(crate) fn service() {
    if !PENDING.swap(false, Ordering::AcqRel) {
        return;
    }
    // No focus frame: the job has no window and starts serving at once.
    if let Err(e) = spawn_prove_job() {
        crate::log::warn!("[PROVE-JOB] not spawned: {:?}", e);
    }
}
