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

//! The prove-job queue (plan/14, plan/15): MkProveJob asks, init's loop
//! spawns the prover bound to the pid that asked, in init's context. One job
//! at a time; neither kind while a prover runs, as job or window.

use spin::Mutex;

use super::ask::ProveAsk;
use super::slots::{Next, Slots};
use crate::userspace::capsule_prove::{prover_live, spawn_prove_job, JobKind};

static QUEUE: Mutex<Slots> = Mutex::new(Slots::new());

/// False when a prover is live, one of its kind waits, or the rate holds.
pub(crate) fn request(ask: ProveAsk) -> bool {
    !prover_live() && QUEUE.lock().ask(ask, crate::time::timestamp_millis())
}

pub(crate) fn has_pending() -> bool {
    QUEUE.lock().waiting()
}

pub(crate) fn service() {
    if prover_live() {
        return;
    }
    let next = QUEUE.lock().next();
    let spawned = match next {
        Some(Next::Prove(pid)) => spawn_prove_job(JobKind::Prove, pid),
        Some(Next::Deliver(pid)) => spawn_prove_job(JobKind::Deliver, pid),
        None => return,
    };
    // No focus frame: the job has no window and starts serving at once.
    if let Err(e) = spawned {
        crate::log::warn!("[PROVE-JOB] not spawned: {:?}", e);
    }
}
