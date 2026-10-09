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

//! The wired engine with its regions also held by name. The boxed engine does
//! everything a prover needs; what it cannot do is hand a recursive verifier
//! each region's transition over the tower. Holding the typed list beside the
//! boxes closes that: same layout, same trace, same constraints — the boxes
//! are built from the same values the names hold — plus the one generic
//! method recursion needs.

use super::super::field::{Felt, Fp, Fp2};
use super::compose_check_gen::GenericTransition;
use super::shield_region::ShieldRegion;
use super::spec::{Air, AirExt};
use super::wired_multi_ext::{GpGroup, WiredMultiExt};
use alloc::vec::Vec;

pub struct WiredMultiGen {
    wired: WiredMultiExt,
    gens: Vec<ShieldRegion>,
}

impl WiredMultiGen {
    /// One constructor, one region list: the boxes the engine stacks are made
    /// from the same values the typed list keeps, so the two views cannot
    /// disagree about what region `i` is.
    pub fn new_kinds(gens: Vec<ShieldRegion>, kinds: &[usize], groups: Vec<GpGroup>) -> Self {
        let boxed = gens.iter().map(|g| g.boxed()).collect();
        WiredMultiGen { wired: WiredMultiExt::new_kinds(boxed, kinds, groups), gens }
    }

    pub fn wired(&self) -> &WiredMultiExt {
        &self.wired
    }

    pub fn trace(&self, traces: &[Vec<Fp>]) -> Vec<Fp> {
        self.wired.trace(traces)
    }

    pub fn group_widths(&self) -> Vec<usize> {
        self.wired.group_widths()
    }

    pub fn region_degrees(&self) -> Vec<usize> {
        self.wired.region_degrees()
    }
}

impl GenericTransition for WiredMultiGen {
    fn transition_gen<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        self.wired.transition_generic(window, periodic, |i, l, p| self.gens[i].transition_gen(l, p))
    }
}

impl Air for WiredMultiGen {
    fn log_trace_len(&self) -> u32 {
        self.wired.log_trace_len()
    }

    fn trace_width(&self) -> usize {
        self.wired.trace_width()
    }

    fn window_size(&self) -> usize {
        self.wired.window_size()
    }

    fn constraint_degree(&self) -> usize {
        self.wired.constraint_degree()
    }

    fn num_transition(&self) -> usize {
        self.wired.num_transition()
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        self.wired.periodic_columns()
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.wired.transition(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        self.wired.boundary()
    }
}

impl AirExt for WiredMultiGen {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.wired.transition_ext(window, periodic)
    }
}
