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


//! Opening a stream to an onion service.

extern crate alloc;

use alloc::vec::Vec;

use nonos_libc::crypto_random;

use crate::cell::RELAY_BEGIN;
use crate::circuit::CircuitStage;
use crate::onion::address::parse;
use crate::onion::cells::begin_onion;
use crate::onion::lookup::Lookup;
use crate::onion::period::Clock;
use crate::path::Relay;
use crate::protocol::CIRCUIT_DIRTY_SECONDS;
use crate::stream::Stream;
use crate::trace;

use super::super::out::{free_id, send_relay, SendError};
use super::super::state::Manager;
use super::job::{OnionJob, PowState, Stage};

/// Lookups at once. Each holds up to two circuits; more would crowd out
/// the general circuits on the one link.
const LOOKUPS_MAX: usize = 4;
/// How long a lookup may take in all. The route_link tunnel gives a stream
/// 45 seconds to open, so a lookup still running past that has no reader.
pub(super) const LOOKUP_SECONDS: u64 = 45;

/// Open a stream to the service `host` names, on `port`, for `owner`.
pub fn open(state: &mut Manager, host: &[u8], port: u16, now: u64, owner: u32) -> Result<u16, SendError> {
    /* A full address carries the service's key; anything else ending in
     * .anyone is a short name, which only the signed list can turn into
     * one (manager/names.rs). */
    let identity = match parse(host) {
        Some(identity) => identity,
        None => super::super::names::resolve(state, host, now)?,
    };
    let id = free_id(state).ok_or(SendError::TableFull)?;

    /* A rendezvous circuit to this service that is still fresh takes the
     * stream at once, as a general circuit does for a host name. */
    let reuse = state.circuits.iter().position(|c| {
        c.service == Some(identity)
            && c.stage == CircuitStage::Open
            && now < c.opened_at.saturating_add(CIRCUIT_DIRTY_SECONDS)
    });
    if let Some(index) = reuse {
        send_relay(state, index, RELAY_BEGIN, id, &begin_onion(port))?;
        let mut stream = Stream::new(id, state.circuits[index].id);
        stream.owner = owner;
        state.streams.push(stream);
        state.next_stream = id;
        trace::say_num(b"onion stream begin sent on a kept circuit", id as u64);
        return Ok(id);
    }

    if state.onion.len() >= LOOKUPS_MAX {
        return Err(SendError::TableFull);
    }
    let clock = Clock { valid_after: state.valid_after, interval: state.fresh_until.saturating_sub(state.valid_after) };
    let lookup = Lookup::new(&identity, &clock).ok_or(SendError::BadOnion)?;
    let hsdirs = responsible(state, &lookup, &clock);
    if hsdirs.is_empty() {
        trace::say(b"onion lookup has no responsible hsdir");
        return Err(SendError::NoCircuit);
    }
    let mut cookie = [0u8; 20];
    if crypto_random(cookie.as_mut_ptr(), cookie.len()) != cookie.len() as i64 {
        return Err(SendError::NoCircuit);
    }

    /* A descriptor already fetched for this period starts the lookup at the
     * rendezvous point; the HSDirs stay in hand in case its introduction
     * points have all moved on. */
    state.desc_cache.prune(lookup.period, now);
    let cached = state.desc_cache.get(&lookup.blinded, lookup.period, now).map(|(points, pow)| (points.to_vec(), pow));
    let from_cache = cached.is_some();
    let (mut intro_points, pow) = cached.unwrap_or_default();
    shuffle(&mut intro_points);

    let mut stream = Stream::new(id, 0);
    stream.owner = owner;
    state.streams.push(stream);
    state.next_stream = id;
    state.onion.push(OnionJob {
        stream: id,
        port,
        lookup,
        stage: if from_cache { Stage::Rendezvous } else { Stage::Directory },
        deadline: now.saturating_add(LOOKUP_SECONDS),
        step_deadline: 0,
        hsdirs,
        intro_points,
        cookie,
        rendezvous_point: None,
        rend: None,
        circuit: None,
        dir_stream: None,
        sent: false,
        ephemeral: None,
        introduced: None,
        outcome: None,
        from_cache,
        rend_ready: false,
        pow,
        pow_state: PowState::Idle,
        unreachable: 0,
        solve_until: None,
    });
    trace::say_num(b"onion lookup started for stream", id as u64);
    Ok(id)
}

/// The responsible HSDirs for `lookup`, in a random order, so the load of
/// one service's lookups is spread across them as a Tor client spreads it.
fn responsible(state: &Manager, lookup: &Lookup, clock: &Clock) -> Vec<Relay> {
    let ring: Vec<&Relay> = state.relays.iter().filter(|r| r.flags.hsdir && r.usable()).collect();
    let ids: Vec<[u8; 32]> = ring.iter().map(|r| r.ed25519_identity).collect();
    let Some(picked) = lookup.hsdirs(clock, state.srv_current, state.srv_previous, &ids) else {
        return Vec::new();
    };
    let mut out: Vec<Relay> = picked.iter().map(|i| ring[*i].clone()).collect();
    shuffle(&mut out);
    out
}

pub(super) fn shuffle<T>(items: &mut [T]) {
    for i in (1..items.len()).rev() {
        let mut bytes = [0u8; 8];
        if crypto_random(bytes.as_mut_ptr(), bytes.len()) != bytes.len() as i64 {
            return;
        }
        let j = (u64::from_le_bytes(bytes) % (i as u64 + 1)) as usize;
        items.swap(i, j);
    }
}
