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

//! The circuit for a statement: with a witness the prover's, without one the
//! verifier's shape, whose constraints and wiring depend on nothing private.
//! Every region is a ShieldRegion kind, so the device proof's blinding and
//! rank certificate apply unchanged.

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::air::{Air, LiveGate, Publics, ShieldRegion, WiredMultiGen};
use nonos_stark::field::Fp;
use stark_proofs::shield::wire::offsets;
use stark_proofs::shield::wire_pack::{groups_enforce, packed_groups};

use super::chains::{a_chain, h_chain};
use super::gates::gates;
use super::layout::{kinds, PUBLICS};
use super::wires::classes;
use crate::params::{LOG_TRACE, MASK_COLUMNS, WIRE_CAP};
use crate::statement::{Statement, Witness};

pub struct Built {
    pub wired: WiredMultiGen,
    pub traces: Vec<Vec<Fp>>,
}

pub fn build(st: &Statement, w: Option<&Witness>) -> Option<Built> {
    let words: Vec<Fp> = st.words().iter().map(|&v| Fp::from_u64(v)).collect();
    let (h, a, g) = (h_chain(w), a_chain(w), gates(w));
    let publics = Publics { log_t: 4, words };
    let mut traces = vec![h.trace(), a.trace()];
    traces.extend(g.iter().map(LiveGate::trace));
    traces.push(publics.trace());
    let mut rows = vec![Air::rows(&h), Air::rows(&a)];
    rows.extend(g.iter().map(Air::rows));
    rows.push(Air::rows(&publics));
    let (off, span) = offsets(&rows);
    let cls = classes(&off);
    let groups = packed_groups(span, &cls, WIRE_CAP);
    if !groups_enforce(&groups, &cls) {
        return None;
    }
    let mut regions = vec![ShieldRegion::Membership(h), ShieldRegion::Membership(a)];
    regions.extend(g.into_iter().map(ShieldRegion::Live));
    regions.push(ShieldRegion::Publics(publics));
    let mut wired = WiredMultiGen::new_kinds(regions, &kinds(), groups)
        .with_mask(MASK_COLUMNS)
        .with_ext_challenges();
    wired.wired_mut().mark_region_public(PUBLICS);
    (Air::log_trace_len(&wired) == LOG_TRACE).then_some(Built { wired, traces })
}

/// The verifier's circuit for a statement.
pub fn shape(st: &Statement) -> Option<WiredMultiGen> {
    build(st, None).map(|b| b.wired)
}
