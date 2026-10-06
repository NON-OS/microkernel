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

//! Reading cells off the link and routing them to their circuit.

use nonos_libc::mk_uptime_ms;

use crate::cell::{Frame, CELL_DESTROY, CELL_RELAY, CELL_RELAY_EARLY};
use crate::circuit::{open, CircuitStage};
use crate::path::guard_pick::died_young;
use crate::trace;

use super::inbound::deliver;
use super::link_lost::lost;
use super::state::Manager;

/// How long the first read waits for a cell. Short, because the serve loop
/// calls back within milliseconds while there is traffic.
const FIRST_WAIT_MS: i64 = 5;
/// Cells taken in one call, and the time one call may spend, so a fast
/// download cannot keep the capsule from answering its callers.
const PUMP_CELLS: usize = 1024;
const PUMP_MS: i64 = 40;
/// SENDMEs are paid every this many cells inside a call, not only after it:
/// a stream's window is 500 cells, and a relay that has sent them all waits
/// for the SENDME before sending more.
const SENDME_EVERY: usize = 32;

/*
 * Every cell waiting on the link is taken, up to the budget. This used to
 * take one cell per call, and the idle path called it once per turn, so a
 * circuit moved at four or five cells a second however fast the relays
 * were: enough for a small page, never for a real one. The time budget is
 * what keeps the serve loop responsive under a circuit delivering hard.
 */
pub fn tick(state: &mut Manager, now: u64) {
    let began = mk_uptime_ms();
    for taken in 0..PUMP_CELLS {
        let wait = if taken == 0 { FIRST_WAIT_MS } else { 0 };
        if !one(state, now, wait) {
            break;
        }
        if taken % SENDME_EVERY == SENDME_EVERY - 1 {
            super::out::sendme_tick(state);
        }
        if mk_uptime_ms().saturating_sub(began) >= PUMP_MS {
            break;
        }
    }
}

/// Take and route one cell. False when none arrived within `wait_ms` or the
/// link was lost, which ends the batch.
fn one(state: &mut Manager, now: u64, wait_ms: i64) -> bool {
    let Some(link) = state.link.as_mut() else { return false };
    let frame = match link.recv(wait_ms) {
        Ok(Some(frame)) => frame,
        Ok(None) => return false,
        Err(_) => {
            charge_guard(state, now);
            lost(state);
            return false;
        }
    };
    let Frame::Fixed(mut cell) = frame else {
        /*
         * A variable length cell after the handshake is padding or a version
         * negotiation cell, with nothing in it that belongs to a circuit.
         */
        return true;
    };
    let Some(index) = state.circuits.iter().position(|c| c.id == cell.circuit) else {
        return true;
    };
    if state.circuits[index].stage == CircuitStage::Handshaking {
        super::circuit_answer::arrived(state, index, *cell, now);
        return true;
    }
    if cell.command == CELL_DESTROY {
        // The first payload byte is the reason (tor-spec 5.4): 1 protocol, 3
        // internal, 5 resource limit, 8 finished, 9 timeout... Without it a
        // circuit torn down for a bad SENDME looks like one that timed out.
        trace::say_two(
            b"circuit destroyed by the far end, reason",
            cell.circuit as u64,
            cell.payload[0] as u64,
        );
        state.circuits[index].stage = CircuitStage::Dead;
        super::end_streams::end_streams(state, index);
        return true;
    }
    if cell.command != CELL_RELAY && cell.command != CELL_RELAY_EARLY {
        return true;
    }
    let Some(opened) = open(&mut state.circuits[index].hops, &mut cell.payload) else {
        /*
         * No hop recognised it. That is not a cell to guess about: a relay cell
         * whose digest matches no hop either came from somewhere else or the
         * chain has desynchronised, and either way the circuit is finished.
         */
        trace::say_num(b"unrecognised cell, circuit dead", cell.circuit as u64);
        state.circuits[index].stage = CircuitStage::Dead;
        super::end_streams::end_streams(state, index);
        return true;
    };
    deliver(state, index, opened.hop, &cell.payload);
    true
}

/// The guard's own link broke: count it against the guard when it broke young.
fn charge_guard(state: &mut Manager, now: u64) {
    if let Some(guard) = state.guard.as_mut() {
        if died_young(guard.tried_at, now) {
            guard.failures = guard.failures.saturating_add(1);
            trace::say(b"guard dropped a young link");
        }
    }
}

