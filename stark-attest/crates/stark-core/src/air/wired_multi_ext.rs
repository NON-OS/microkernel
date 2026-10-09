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

//! The money-grade wired engine with the copy constraint split across several
//! grand-product columns. Routing every shared value through one permutation makes
//! that constraint's degree the number of wired columns, which for a large assembly
//! blows up the evaluation domain and the on-chain composition check. Splitting the
//! bindings into independent groups, one grand-product column each, keeps every
//! constraint at the size of its group plus a constant, so the AIR degree stays at
//! the region maximum. Same soundness, same bindings, cheap to verify. Layout,
//! region transitions, and each group's product come from `fusion`.

use super::super::field::{Felt, Fp, Fp2};
use super::fusion::{self, Stack};
use super::spec::{Air, AirExt};
use alloc::boxed::Box;
use alloc::vec::Vec;

/// One copy-constraint group: the columns it binds, the permutation over their
/// cells, and its challenges. Each group is an independent grand product.
pub struct GpGroup {
    pub wired_cols: Vec<usize>,
    pub sigma: Vec<usize>,
    pub beta: Fp,
    pub gamma: Fp,
}

pub struct WiredMultiExt {
    regions: Vec<Box<dyn AirExt>>,
    stack: Stack,
    groups: Vec<GpGroup>,
    /// The identity column `r * k + j` depends only on a group's width, and the
    /// product selector is the same column for every group, so both are emitted
    /// once and shared. Only sigma is per group.
    row_idx: usize,
    sig_base: Vec<usize>,
    sel_idx: usize,
    region_transitions: usize,
}

impl WiredMultiExt {
    /// Width of each running product. The widest sets the degree, which sets the
    /// evaluation domain.
    pub fn group_widths(&self) -> Vec<usize> {
        self.groups.iter().map(|g| g.wired_cols.len()).collect()
    }

    /// Per region degree. The AIR takes the larger of this and the widest product,
    /// so moving one alone moves nothing.
    pub fn region_degrees(&self) -> Vec<usize> {
        self.regions.iter().map(|r| r.constraint_degree()).collect()
    }

    pub fn new(regions: Vec<Box<dyn AirExt>>, groups: Vec<GpGroup>) -> WiredMultiExt {
        let kinds: Vec<usize> = (0..regions.len()).collect();
        WiredMultiExt::new_kinds(regions, &kinds, groups)
    }

    /// `kinds[i]` names region `i`'s kind. Instances of one kind must run equal
    /// constraints over an equal periodic pattern; they then share one selector
    /// and one set of columns instead of carrying an identical copy each.
    pub fn new_kinds(
        regions: Vec<Box<dyn AirExt>>,
        kinds: &[usize],
        groups: Vec<GpGroup>,
    ) -> WiredMultiExt {
        let stack = Stack::of_kinds(&regions, kinds);
        // A kind runs one instance's constraints over every instance's rows, so
        // instances that are not the same AIR swap one region's rules for
        // another's. The caller declares kinds, so check the caller.
        for (i, &k) in kinds.iter().enumerate() {
            let rep = stack.kind_first[k];
            assert!(
                regions[i].trace_width() == regions[rep].trace_width()
                    && regions[i].window_size() == regions[rep].window_size()
                    && regions[i].log_trace_len() == regions[rep].log_trace_len()
                    && regions[i].num_transition() == regions[rep].num_transition()
                    && regions[i].constraint_degree() == regions[rep].constraint_degree()
                    && regions[i].periodic_columns() == regions[rep].periodic_columns(),
                "region {i} is declared kind {k} but does not match instance {rep}"
            );
        }
        let region_slots = stack.kind_slot.last().copied().unwrap_or(0)
            + stack.kind_first.last().map(|&i| regions[i].periodic_columns().len()).unwrap_or(0);
        let base = stack.n_kinds + region_slots;
        let sel_idx = base;
        // The identity a cell is compared against is r * k + j, which is linear in
        // the row, so one column of r serves every group and lane. It used to be a
        // column per lane per distinct width: 21 columns on the recursion, each the
        // full trace length, to carry what multiply and add already give.
        let row_idx = base + 1;
        let mut s = base + 2;
        let mut sig_base = Vec::with_capacity(groups.len());
        for grp in &groups {
            sig_base.push(s);
            s += grp.wired_cols.len();
        }
        for (g, grp) in groups.iter().enumerate() {
            let k = grp.wired_cols.len();
            stack.assert_bound_below_close(&grp.sigma, k, &alloc::format!("group {g}"));
        }
        let mut region_transitions = 0usize;
        for region in &regions {
            region_transitions = region_transitions.max(region.num_transition());
        }
        WiredMultiExt { regions, stack, groups, row_idx, sig_base, sel_idx, region_transitions }
    }

    fn stride(&self) -> usize {
        self.stack.width + self.groups.len()
    }

    fn closes_at(&self) -> usize {
        self.stack.closes_at()
    }

    fn ratio(&self, group: &GpGroup, row: &[Fp], r: usize) -> Fp {
        let k = group.wired_cols.len();
        let (b, gm) = (group.beta, group.gamma);
        let mut num = Fp::ONE;
        let mut den = Fp::ONE;
        for (j, &col) in group.wired_cols.iter().enumerate() {
            let v = row[col];
            let id = r * k + j;
            num = num * (v + b * Fp::from_u64(id as u64) + gm);
            den = den * (v + b * Fp::from_u64(group.sigma[id] as u64) + gm);
        }
        num * den.inv()
    }

    /// The witness: regions in the low columns, each group's running product in one
    /// column above them.
    pub fn trace(&self, traces: &[Vec<Fp>]) -> Vec<Fp> {
        let stride = self.stride();
        let total = 1usize << self.log_trace_len();
        let mut trace = fusion::place_traces(&self.stack, &self.regions, stride, total, traces);
        let span = self.closes_at();
        for (g, group) in self.groups.iter().enumerate() {
            let z_col = self.stack.width + g;
            let mut z = Fp::ONE;
            for r in 0..total {
                trace[r * stride + z_col] = z;
                if r < span {
                    let base = r * stride;
                    z = z * self.ratio(group, &trace[base..base + stride], r);
                }
            }
        }
        trace
    }

    fn group_product<F: Felt>(&self, g: usize, group: &GpGroup, window: &[F], periodic: &[F]) -> F {
        let b = F::from_base(group.beta);
        let gm = F::from_base(group.gamma);
        let stride = self.stride();
        let width = self.stack.width;
        let z = window[width + g];
        let z_next = window[stride + width + g];
        let sgb = self.sig_base[g];
        let kf = F::from_base(Fp::from_u64(group.wired_cols.len() as u64));
        let row = periodic[self.row_idx];
        let mut num = F::ONE;
        let mut den = F::ONE;
        for (j, &col) in group.wired_cols.iter().enumerate() {
            let v = window[col];
            let id = row * kf + F::from_base(Fp::from_u64(j as u64));
            let sig = periodic[sgb + j];
            num = num * (v + b * id + gm);
            den = den * (v + b * sig + gm);
        }
        let gp_sel = periodic[self.sel_idx];
        let product = z_next * den - z * num;
        let carry = z_next - z;
        gp_sel * product + (F::ONE - gp_sel) * carry
    }

    /// The full transition over any field, with the caller supplying each
    /// region's constraint evaluation. This is how a recursive verifier
    /// recomputes the whole wired AIR inside its own circuit: the layout,
    /// selectors and grand products replay here, and the closure dispatches to
    /// each region's own generic transition, which the boxed form cannot carry.
    pub fn transition_generic<F: Felt>(
        &self,
        window: &[F],
        periodic: &[F],
        region: impl Fn(usize, &[F], &[F]) -> Vec<F>,
    ) -> Vec<F> {
        let mut out = fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stride(),
            window,
            periodic,
            region,
        );
        self.append_groups(&mut out, window, periodic);
        out
    }

    fn append_groups<F: Felt>(&self, out: &mut [F], window: &[F], periodic: &[F]) {
        for (g, group) in self.groups.iter().enumerate() {
            out[self.region_transitions + g] = self.group_product(g, group, window, periodic);
        }
    }
}

impl AirExt for WiredMultiExt {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        let mut out = fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stride(),
            window,
            periodic,
            |i, l, p| self.regions[i].transition_ext(l, p),
        );
        self.append_groups(&mut out, window, periodic);
        out
    }
}

impl Air for WiredMultiExt {
    fn log_trace_len(&self) -> u32 {
        self.stack.log_trace_len()
    }

    fn trace_width(&self) -> usize {
        self.stride()
    }

    fn window_size(&self) -> usize {
        self.stack.window
    }

    fn constraint_degree(&self) -> usize {
        let mut d = 1usize;
        for region in &self.regions {
            d = d.max(region.constraint_degree());
        }
        let max_group = self.groups.iter().map(|g| g.wired_cols.len()).max().unwrap_or(0);
        (d + 2).max(max_group + 2)
    }

    fn num_transition(&self) -> usize {
        self.region_transitions + self.groups.len()
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let total = 1usize << self.log_trace_len();
        let span = self.closes_at();
        let mut cols = fusion::base_periodic(&self.stack, &self.regions, total);
        let mut gp_sel = alloc::vec![Fp::ZERO; total];
        for item in gp_sel.iter_mut().take(span) {
            *item = Fp::ONE;
        }
        cols.push(gp_sel);
        let mut row = alloc::vec![Fp::ZERO; total];
        for (r, slot) in row.iter_mut().enumerate().take(span) {
            *slot = Fp::from_u64(r as u64);
        }
        cols.push(row);
        for group in &self.groups {
            let k = group.wired_cols.len();
            for j in 0..k {
                let mut sig = alloc::vec![Fp::ZERO; total];
                for (r, slot) in sig.iter_mut().enumerate().take(span) {
                    *slot = Fp::from_u64(group.sigma[r * k + j] as u64);
                }
                cols.push(sig);
            }
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        let mut out = fusion::combine(
            &self.stack,
            &self.regions,
            self.num_transition(),
            self.stride(),
            window,
            periodic,
            |i, l, p| self.regions[i].transition(l, p),
        );
        self.append_groups(&mut out, window, periodic);
        out
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        let mut b = fusion::base_boundary(&self.stack, &self.regions);
        let span = self.closes_at();
        for g in 0..self.groups.len() {
            b.push((self.stack.width + g, 0, Fp::ONE));
            b.push((self.stack.width + g, span, Fp::ONE));
        }
        b
    }
}
