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


//! Building and tearing down the circuits a lookup uses.

extern crate alloc;

use nonos_libc::mk_uptime_ms;

use crate::circuit::{ask_create, client_circuit_id, destroy, Circuit, CircuitStage, Pending, Purpose, HOP_MS};
use crate::path::{draw_middle, Relay};
use crate::stream::ONION;
use crate::trace;

use super::super::state::Manager;

/// Start a three hop circuit through the guard to `target`, owned by a
/// lookup. Its hops are finished by the pump like any other; a failed build
/// destroys only it (circuit_answer::failed). `None` when there is no link,
/// `target` is the guard, or no middle can be drawn.
pub(super) fn start(state: &mut Manager, target: &Relay) -> Option<u32> {
    let guard = state.guard.as_ref()?.relay.clone();
    if guard.rsa_identity == target.rsa_identity {
        return None;
    }
    let middle = draw_middle(&state.relays, &state.weights, &[&guard, target])?;
    let id = client_circuit_id(state.next_circuit);
    let link = state.link.as_mut()?;
    state.next_circuit = state.next_circuit.wrapping_add(1);
    let handshake = ask_create(link, id, &guard).ok()?;
    let mut circuit = Circuit::new(id, alloc::vec![guard, middle, target.clone()]);
    circuit.purpose = Purpose::Onion;
    circuit.pending = Some(Pending { handshake, deadline: mk_uptime_ms().saturating_add(HOP_MS) });
    state.circuits.push(circuit);
    trace::say_num(b"onion circuit started", id as u64);
    Some(id)
}

/// Where circuit `id` stands: `None` when it is gone or dead, or its hop
/// in flight has passed its deadline (which is the lookup's to act on, as
/// circuit_tick only times general circuits).
pub(super) fn open(state: &Manager, id: u32) -> Option<bool> {
    let circuit = state.circuits.iter().find(|c| c.id == id)?;
    match circuit.stage {
        CircuitStage::Open => Some(true),
        CircuitStage::Dead => None,
        CircuitStage::Handshaking => {
            let late = circuit.pending.as_ref().is_none_or(|p| mk_uptime_ms() >= p.deadline);
            (!late).then_some(false)
        }
    }
}

/// DESTROY circuit `id` and drop it, ending any stream still on it.
pub(super) fn drop_circuit(state: &mut Manager, id: u32) {
    if !state.circuits.iter().any(|c| c.id == id) {
        return;
    }
    if let Some(link) = state.link.as_mut() {
        let _ = link.send(&destroy(id).encode());
    }
    for stream in state.streams.iter_mut().filter(|s| s.circuit == id) {
        stream.end_with_circuit();
    }
    /* A lookup's own BEGIN_DIR stream has no caller to read its end. */
    state.streams.retain(|s| !(s.circuit == id && s.owner == ONION));
    state.circuits.retain(|c| c.id != id);
}
