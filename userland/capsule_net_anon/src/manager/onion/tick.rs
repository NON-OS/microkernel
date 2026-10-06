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


//! Running every lookup one step, and ending the ones that are done.

use crate::stream::StreamStage;
use crate::trace;

use super::super::state::Manager;
use super::circuits::drop_circuit;
use super::job::{OnionJob, Outcome, Stage};

/// TIMEOUT: the lookup ran past LOOKUP_SECONDS.
const REASON_TIMEOUT: u8 = 7;

pub fn tick(state: &mut Manager, now: u64) {
    super::super::names::tick(state, now);
    if state.onion.is_empty() {
        return;
    }
    let jobs = core::mem::take(&mut state.onion);
    for mut job in jobs {
        match advance(state, &mut job, now) {
            None => state.onion.push(job),
            Some(Outcome::Connected) => connected(state, &job, now),
            Some(Outcome::Failed(reason)) => failed(state, job, reason),
        }
    }
}

fn advance(state: &mut Manager, job: &mut OnionJob, now: u64) -> Option<Outcome> {
    if let Some(outcome) = job.outcome {
        return Some(outcome);
    }
    /* The caller closed its stream, or the link went and took it: nobody is
     * waiting, so the lookup's circuits go with it. */
    if !state.streams.iter().any(|s| s.id == job.stream) {
        trace::say_num(b"onion lookup abandoned with its stream", job.stream as u64);
        return Some(Outcome::Failed(0));
    }
    if now >= job.deadline {
        trace::say_num(b"onion lookup ran out of time", job.stream as u64);
        return Some(Outcome::Failed(REASON_TIMEOUT));
    }
    let stepped = match job.stage {
        Stage::Directory => super::directory::step(state, job, now),
        _ => super::rendezvous::step(state, job, now),
    };
    stepped.err().map(Outcome::Failed)
}

/// The rendezvous circuit now carries the stream; it stays, keyed by the
/// service, for the next stream to it. Anything else the lookup held goes.
fn connected(state: &mut Manager, job: &OnionJob, now: u64) {
    if let Some(id) = job.circuit {
        drop_circuit(state, id);
    }
    if let Some(circuit) = job.rend.and_then(|id| state.circuits.iter_mut().find(|c| c.id == id)) {
        circuit.opened_at = now;
    }
}

fn failed(state: &mut Manager, job: OnionJob, reason: u8) {
    for id in [job.circuit, job.rend].into_iter().flatten() {
        drop_circuit(state, id);
    }
    if let Some(stream) = state.streams.iter_mut().find(|s| s.id == job.stream) {
        if stream.stage == StreamStage::Opening {
            stream.stage = StreamStage::Ended(reason.max(1));
        }
    }
}
