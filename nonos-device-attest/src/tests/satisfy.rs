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

//! Witness checks without FRI, over the public `Air` trait: every transition
//! vanishes and every boundary holds. What the audit and the checkpoint tests
//! change a cell against.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::crypto::stark::field::Fp;

use super::fixture::fixture;
use crate::circuit::{build, Built};

pub fn honest() -> (Built, Vec<Fp>) {
    let f = fixture();
    let b = build(&f.st, Some(&f.w)).expect("the fixture builds");
    let witness = b.wired.trace(&b.traces);
    (b, witness)
}

/// The first violation, or `None` when the witness satisfies every rule.
pub fn violation(air: &impl Air, t: &[Fp]) -> Option<(usize, usize)> {
    let w = air.trace_width();
    let ws = air.window_size();
    let n = 1usize << air.log_trace_len();
    let per = air.periodic_columns();
    for r in 0..=n - ws {
        let p: Vec<Fp> = per.iter().map(|c| c[r]).collect();
        if let Some(lane) =
            air.transition(&t[r * w..(r + ws) * w], &p).iter().position(|v| *v != Fp::ZERO)
        {
            return Some((r, lane));
        }
    }
    let nt = air.num_transition();
    air.boundary().into_iter().find(|&(c, r, v)| t[r * w + c] != v).map(|(c, r, _)| (r, nt + c))
}

/// Region instances as (first row, last live row, kind), in row order, from
/// the kind selector columns.
pub fn runs(air: &impl Air, kinds: usize) -> Vec<(usize, usize, usize)> {
    let per = air.periodic_columns();
    let n = 1usize << air.log_trace_len();
    let mut out = Vec::new();
    for (k, sel) in per.iter().enumerate().take(kinds) {
        let mut r = 0;
        while r < n {
            if sel[r] == Fp::ONE {
                let s = r;
                while r < n && sel[r] == Fp::ONE {
                    r += 1;
                }
                out.push((s, r - 1, k));
            } else {
                r += 1;
            }
        }
    }
    out.sort_unstable();
    out
}
