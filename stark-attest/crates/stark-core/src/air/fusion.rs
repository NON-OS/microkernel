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

//! Shared region-stacking for the money-grade fused and wired engines, so the
//! offset layout, the selected region-transition combination, and the copy
//! constraint grand product live in one place instead of being copied per engine.
//! The transition helpers are generic over `Felt`, so a single implementation
//! serves both the base-field composition and the extension out-of-domain
//! evaluation; the caller passes a closure that reads each region's constraints.

use super::super::field::{Felt, Fp};
use super::spec::AirExt;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Lay regions out end to end and report each one's row offset and the total.
///
/// The one place this arithmetic lives. It was written out five times before,
/// in the two wired engines, the fused one, the shield batch and a test helper,
/// and they drifted the moment a region stopped rounding its own length: some
/// counted the rows a region occupies and some its padded trace, so bindings
/// addressed cells past the row the product closes on.
pub fn region_offsets(regions: &[Box<dyn AirExt>]) -> (Vec<usize>, usize) {
    let mut offsets = Vec::with_capacity(regions.len());
    let mut row = 0usize;
    for region in regions {
        offsets.push(row);
        row += region.rows();
    }
    (offsets, row)
}

/// The computed placement of a stack of regions in one trace.
pub(super) struct Stack {
    pub offsets: Vec<usize>,
    pub slot_offsets: Vec<usize>,
    pub width: usize,
    pub window: usize,
    pub log_span: u32,
    /// Rows the regions actually occupy, before any rounding. The wired engines
    /// size their trace from this plus the row the running product closes on,
    /// rather than rounding the span and then doubling the result.
    pub rows: usize,
    /// Instance to kind. Instances of one kind run the same constraints over the
    /// same periodic pattern, so they share one selector and one set of columns
    /// instead of carrying an identical copy each.
    pub kind_of: Vec<usize>,
    pub n_kinds: usize,
    pub kind_slot: Vec<usize>,
    pub kind_first: Vec<usize>,
}

impl Stack {
    /// Every region its own kind: the layout as it was before kinds existed.
    pub fn of(regions: &[Box<dyn AirExt>]) -> Stack {
        Stack::of_kinds(regions, &(0..regions.len()).collect::<Vec<usize>>())
    }

    /// Lay regions out end to end, tracking each one's row offset and periodic-slot
    /// offset, the widest width, the largest window, and the rounded total height.
    /// `kinds[i]` names region `i`'s kind; equal kinds must run equal constraints.
    pub fn of_kinds(regions: &[Box<dyn AirExt>], kinds: &[usize]) -> Stack {
        let mut offsets = Vec::with_capacity(regions.len());
        let mut slot_offsets = Vec::with_capacity(regions.len());
        let mut row = 0usize;
        let mut slot = 0usize;
        let mut width = 1usize;
        let mut window = 2usize;
        for region in regions {
            offsets.push(row);
            slot_offsets.push(slot);
            row += region.rows();
            slot += region.periodic_columns().len();
            width = width.max(region.trace_width());
            window = window.max(region.window_size());
        }
        let n_kinds = kinds.iter().max().map(|m| m + 1).unwrap_or(0);
        let mut kind_first = alloc::vec![usize::MAX; n_kinds];
        for (i, &k) in kinds.iter().enumerate() {
            if kind_first[k] == usize::MAX {
                kind_first[k] = i;
            }
        }
        let mut kind_slot = Vec::with_capacity(n_kinds);
        let mut s = 0usize;
        for &first in &kind_first {
            kind_slot.push(s);
            s += regions[first].periodic_columns().len();
        }
        let log_span = row.next_power_of_two().trailing_zeros();
        Stack {
            offsets,
            slot_offsets,
            width,
            window,
            log_span,
            rows: row,
            kind_of: kinds.to_vec(),
            n_kinds,
            kind_slot,
            kind_first,
        }
    }

    pub fn span(&self) -> usize {
        1usize << self.log_span
    }

    /// Where a running product closes: the last row the regions occupy. Nothing
    /// above it to absorb. Both wired engines size from here so they cannot drift.
    pub fn closes_at(&self) -> usize {
        self.rows
    }

    /// One row past the close, rounded up. This used to round `log_span` and then
    /// double it, which cost the recursion 2^19 rows to hold 139522.
    pub fn log_trace_len(&self) -> u32 {
        (self.closes_at() + 1).next_power_of_two().trailing_zeros()
    }

    pub fn height(regions: &[Box<dyn AirExt>], i: usize) -> usize {
        regions[i].rows()
    }

    /// Cells above the closing row are never read, so a class up there is dropped
    /// in silence. Regions sit below it, so this holds; check it anyway, since a
    /// lost binding is the failure the engine exists to catch.
    pub fn assert_bound_below_close(&self, sigma: &[usize], k: usize, what: &str) {
        let close = self.closes_at();
        for (pos, &img) in sigma.iter().enumerate().skip(close * k) {
            assert!(pos == img, "{what} binds cell {pos} above the closing row {close}");
        }
    }
}

/// Combine the selected region transitions into the fused constraint vector.
/// `stride` is the trace width (plus one when a product column follows), and
/// `region_transition` reads region `i`'s constraints over the field `F`, so the
/// same body serves `transition` (base) and `transition_ext` (extension).
pub(super) fn combine<F: Felt>(
    stack: &Stack,
    regions: &[Box<dyn AirExt>],
    num_transition: usize,
    stride: usize,
    window: &[F],
    periodic: &[F],
    region_transition: impl Fn(usize, &[F], &[F]) -> Vec<F>,
) -> Vec<F> {
    let sel = stack.n_kinds;
    let mut out = alloc::vec![F::ZERO; num_transition];
    for k in 0..stack.n_kinds {
        let i = stack.kind_first[k];
        let region = &regions[i];
        let s = periodic[k];
        let w = region.trace_width();
        let ws = region.window_size();
        let mut local = Vec::with_capacity(w * ws);
        for step in 0..ws {
            local.extend_from_slice(&window[step * stride..step * stride + w]);
        }
        let base = sel + stack.kind_slot[k];
        let slots = region.periodic_columns().len();
        let values = region_transition(i, &local, &periodic[base..base + slots]);
        for (c, v) in values.into_iter().enumerate() {
            out[c] = out[c] + s * v;
        }
    }
    out
}

/// The copy-constraint grand-product transition value over `F`: in the product
/// region the running product `z` accumulates `(v + beta*id + gamma)` against
/// `(v + beta*sigma + gamma)` over the wired columns; elsewhere it is carried.
/// `gp` is the first grand-product periodic slot.
#[allow(clippy::too_many_arguments)]
pub(super) fn grand_product<F: Felt>(
    wired_cols: &[usize],
    beta: Fp,
    gamma: Fp,
    width: usize,
    stride: usize,
    gp: usize,
    window: &[F],
    periodic: &[F],
) -> F {
    let b = F::from_base(beta);
    let g = F::from_base(gamma);
    let z = window[width];
    let z_next = window[stride + width];
    let mut num = F::ONE;
    let mut den = F::ONE;
    for (j, &col) in wired_cols.iter().enumerate() {
        let v = window[col];
        let id = periodic[gp + 2 * j];
        let sig = periodic[gp + 2 * j + 1];
        num = num * (v + b * id + g);
        den = den * (v + b * sig + g);
    }
    let gp_sel = periodic[gp + 2 * wired_cols.len()];
    let product = z_next * den - z * num;
    let carry = z_next - z;
    gp_sel * product + (F::ONE - gp_sel) * carry
}

/// The selector and region periodic columns shared by both engines: one selector
/// per region (on its rows but its last), then each region's own periodic columns
/// at its offset. Grand-product columns, if any, are appended by the caller.
pub(super) fn base_periodic(
    stack: &Stack,
    regions: &[Box<dyn AirExt>],
    total: usize,
) -> Vec<Vec<Fp>> {
    let mut cols: Vec<Vec<Fp>> = Vec::new();
    for k in 0..stack.n_kinds {
        let mut col = alloc::vec![Fp::ZERO; total];
        for i in instances(stack, k) {
            let off = stack.offsets[i];
            let h = Stack::height(regions, i);
            for item in col.iter_mut().take(off + h - 1).skip(off) {
                *item = Fp::ONE;
            }
        }
        cols.push(col);
    }
    for k in 0..stack.n_kinds {
        for region_col in regions[stack.kind_first[k]].periodic_columns() {
            let mut col = alloc::vec![Fp::ZERO; total];
            for i in instances(stack, k) {
                let off = stack.offsets[i];
                let h = Stack::height(regions, i);
                col[off..off + h].copy_from_slice(&region_col[..h]);
            }
            cols.push(col);
        }
    }
    cols
}

fn instances(stack: &Stack, kind: usize) -> impl Iterator<Item = usize> + '_ {
    stack.kind_of.iter().enumerate().filter(move |(_, &k)| k == kind).map(|(i, _)| i)
}

/// Each region's boundaries lifted to their fused rows.
pub(super) fn base_boundary(stack: &Stack, regions: &[Box<dyn AirExt>]) -> Vec<(usize, usize, Fp)> {
    let mut b = Vec::new();
    for (i, region) in regions.iter().enumerate() {
        let off = stack.offsets[i];
        for (col, row, val) in region.boundary() {
            b.push((col, off + row, val));
        }
    }
    b
}

/// The fused witness: each region's row-major trace placed at its offset in the
/// low `stride` columns. `traces[i]` is region `i`'s own trace.
pub(super) fn place_traces(
    stack: &Stack,
    regions: &[Box<dyn AirExt>],
    stride: usize,
    total: usize,
    traces: &[Vec<Fp>],
) -> Vec<Fp> {
    let mut fused = alloc::vec![Fp::ZERO; total * stride];
    for (i, region) in regions.iter().enumerate() {
        let w = region.trace_width();
        let off = stack.offsets[i];
        let h = Stack::height(regions, i);
        for r in 0..h {
            let base = (off + r) * stride;
            fused[base..base + w].copy_from_slice(&traces[i][r * w..r * w + w]);
        }
    }
    fused
}
