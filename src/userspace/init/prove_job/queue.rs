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

//! The prove-job queue: MkProveJob asks, init's loop
//! spawns the prover bound to the pid that asked, in init's context. One job
//! at a time; PROVE first; neither while a prover runs, as job or window.

use spin::Mutex;

use super::rate::Rate;
use crate::userspace::capsule_prove::{prover_live, spawn_prove_job, JobKind};

/// The store's pid; or the supervisor's pid and its measurement.
pub(crate) enum ProveAsk {
    Prove(u32),
    Deliver(u32, [u8; 32]),
}

struct Queue {
    prove: Option<u32>,
    deliver: Option<u32>,
    rate: Rate,
}

static QUEUE: Mutex<Queue> = Mutex::new(Queue { prove: None, deliver: None, rate: Rate::new() });

/// False when one of its kind waits, a prover is live, or the rate holds.
pub(crate) fn request(ask: ProveAsk) -> bool {
    if prover_live() {
        return false;
    }
    let now = crate::time::timestamp_millis();
    let mut q = QUEUE.lock();
    match ask {
        ProveAsk::Prove(pid) if q.prove.is_none() => q.prove = Some(pid),
        ProveAsk::Deliver(pid, m) if q.deliver.is_none() && q.rate.take(&m, now) => {
            q.deliver = Some(pid)
        }
        _ => return false,
    }
    true
}

pub(crate) fn has_pending() -> bool {
    let q = QUEUE.lock();
    q.prove.is_some() || q.deliver.is_some()
}

pub(crate) fn service() {
    if prover_live() {
        return;
    }
    let next = {
        let mut q = QUEUE.lock();
        let prove = q.prove.take().map(|p| (JobKind::Prove, p));
        prove.or_else(|| q.deliver.take().map(|p| (JobKind::Deliver, p)))
    };
    // No focus frame: the job has no window and starts serving at once.
    if let Some(Err(e)) = next.map(|(kind, pid)| spawn_prove_job(kind, pid)) {
        crate::log::warn!("[PROVE-JOB] not spawned: {:?}", e);
    }
}
