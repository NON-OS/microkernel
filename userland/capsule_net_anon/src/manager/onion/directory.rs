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


//! Fetching the descriptor: one responsible HSDir after another, each over
//! its own circuit and a BEGIN_DIR stream (hs_client_fetch_v3_desc).

use crate::crypto::x25519;
use crate::onion::cache::CacheKey;
use crate::onion::cells::RELAY_BEGIN_DIR;
use crate::onion::desc::{decode, DESC_MAX};
use crate::onion::fetch::{body, request, Answer};
use crate::stream::{Stream, StreamStage, ONION};
use crate::trace;

use super::super::out::{free_id, send_data, send_relay};
use super::super::state::Manager;
use super::circuits::{drop_circuit, open, start};
use super::job::{OnionJob, PowState, Stage};
use super::open::shuffle;

/// One HSDir gets this long to answer, its circuit included.
const HSDIR_SECONDS: u64 = 15;
/// RESOLVEFAILED: no HSDir served a descriptor that checked out.
pub(super) const REASON_RESOLVEFAILED: u8 = 2;

/// Advance the fetch. `Err` with the END reason when every responsible
/// HSDir has been asked and none served a usable descriptor.
pub(super) fn step(state: &mut Manager, job: &mut OnionJob, now: u64) -> Result<(), u8> {
    let Some(id) = job.circuit else {
        return next_hsdir(state, job, now);
    };
    if now >= job.step_deadline {
        trace::say_num(b"onion hsdir did not answer in time", id as u64);
        return give_up_hsdir(state, job, now);
    }
    match open(state, id) {
        None => return give_up_hsdir(state, job, now),
        Some(false) => return Ok(()),
        Some(true) => {}
    }
    let Some(stream_id) = job.dir_stream else {
        return begin_dir(state, job, id, now);
    };
    let Some(stream) = state.streams.iter().find(|s| s.id == stream_id) else {
        return give_up_hsdir(state, job, now);
    };
    match stream.stage {
        StreamStage::Opening => Ok(()),
        StreamStage::Open if !job.sent => {
            if send_data(state, stream_id, &request(&job.lookup.blinded)).is_err() {
                return give_up_hsdir(state, job, now);
            }
            job.sent = true;
            Ok(())
        }
        StreamStage::Open if stream.inbound.len() > DESC_MAX + 4096 => give_up_hsdir(state, job, now),
        StreamStage::Open => Ok(()),
        StreamStage::Ended(_) => finished(state, job, stream_id, now),
    }
}

fn next_hsdir(state: &mut Manager, job: &mut OnionJob, now: u64) -> Result<(), u8> {
    while let Some(hsdir) = job.hsdirs.pop() {
        if let Some(id) = start(state, &hsdir) {
            job.circuit = Some(id);
            job.dir_stream = None;
            job.sent = false;
            job.step_deadline = now.saturating_add(HSDIR_SECONDS);
            return Ok(());
        }
    }
    trace::say(b"onion descriptor not found at any responsible hsdir");
    Err(REASON_RESOLVEFAILED)
}

fn begin_dir(state: &mut Manager, job: &mut OnionJob, circuit: u32, now: u64) -> Result<(), u8> {
    let Some(stream_id) = free_id(state) else { return Ok(()) };
    let Some(index) = state.circuits.iter().position(|c| c.id == circuit) else {
        return give_up_hsdir(state, job, now);
    };
    if send_relay(state, index, RELAY_BEGIN_DIR, stream_id, &[]).is_err() {
        return give_up_hsdir(state, job, now);
    }
    let mut stream = Stream::new(stream_id, circuit);
    stream.owner = ONION;
    state.streams.push(stream);
    state.next_stream = stream_id;
    job.dir_stream = Some(stream_id);
    Ok(())
}

/// The HSDir closed the stream: whatever it sent is the whole answer.
fn finished(state: &mut Manager, job: &mut OnionJob, stream_id: u16, now: u64) -> Result<(), u8> {
    let raw = state
        .streams
        .iter_mut()
        .find(|s| s.id == stream_id)
        .map(|s| core::mem::take(&mut s.inbound))
        .unwrap_or_default();
    let decoded = match body(&raw) {
        Ok(text) => decode(text, &job.lookup.blinded, &job.lookup.subcredential, now, |ephemeral| {
            let key = state.client_keys.iter().find(|(k, _)| k.identity == job.lookup.identity)?;
            x25519(key.0.secret(), ephemeral).ok()
        })
        .map_err(|why| {
            trace::say(why.said());
        }),
        Err(Answer::NotFound) => {
            trace::say(b"onion hsdir holds no descriptor for the service");
            Err(())
        }
        Err(_) => {
            trace::say(b"onion hsdir answer not usable");
            Err(())
        }
    };
    let Ok(descriptor) = decoded else {
        return give_up_hsdir(state, job, now);
    };
    if let Some(id) = job.circuit.take() {
        drop_circuit(state, id);
    }
    job.dir_stream = None;
    let lookup = &job.lookup;
    let key = CacheKey { identity: lookup.identity, blinded: lookup.blinded, period: lookup.period };
    state.desc_cache.insert(key, descriptor.revision, descriptor.lifetime, now, descriptor.intro_points.clone(), descriptor.pow);
    job.intro_points = descriptor.intro_points;
    /* A refetched descriptor may carry a new seed; a puzzle under the old
     * one is dropped and the next introduction starts over. */
    if job.pow != descriptor.pow {
        job.pow_state = PowState::Idle;
    }
    job.pow = descriptor.pow;
    shuffle(&mut job.intro_points);
    job.from_cache = false;
    /* A rendezvous point established before a refetch is still good. */
    job.stage = if job.rend_ready { Stage::Introduce } else { Stage::Rendezvous };
    job.sent = false;
    trace::say_num(b"onion descriptor checked, introduction points", job.intro_points.len() as u64);
    Ok(())
}

fn give_up_hsdir(state: &mut Manager, job: &mut OnionJob, now: u64) -> Result<(), u8> {
    if let Some(id) = job.circuit.take() {
        drop_circuit(state, id);
    }
    if let Some(stream_id) = job.dir_stream.take() {
        state.streams.retain(|s| s.id != stream_id);
    }
    next_hsdir(state, job, now)
}
