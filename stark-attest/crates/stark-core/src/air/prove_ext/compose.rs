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

use super::super::super::field::{Fp, Fp2};
use super::super::composition::compose_ext;
use super::super::spec::AirExt;
use super::coset::extend;
use super::setup::Domain;
use alloc::vec::Vec;

/// Rows go to threads in blocks: a block allocates its window once, clears and
/// reuses it, and carries its own point forward multiplicatively.
pub(in crate::air) const BLOCK: usize = 1024;

/// The composition over the whole domain, walked coset by coset.
///
/// The window at position `j` reads `(j + k * blowup) % n`, which shares j's
/// residue mod blowup: a window never leaves its coset, it wraps to row
/// `(i + k) % t` of the same one. That wrap is what makes streaming exact, and
/// it is the only fact this function rests on.
pub(in crate::air) fn over_domain<A: AirExt>(
    air: &A,
    d: &Domain,
    trace: &[Vec<Fp>],
    periodic: &[Vec<Fp>],
    coeffs: &[Fp2],
) -> Vec<Fp2> {
    let mut comp_d = alloc::vec![Fp2::ZERO; d.n];
    for c in 0..d.blowup {
        let cols = extend(trace, d, c);
        let per = extend(periodic, d, c);
        let shift_c = d.coset_shift(c);
        let blocks = d.t.div_ceil(BLOCK);
        let parts = crate::par::map_index(blocks, |b| {
            let (lo, hi) = (b * BLOCK, ((b + 1) * BLOCK).min(d.t));
            let mut window: Vec<Fp2> = Vec::with_capacity(d.window * d.width);
            let mut periodic_i: Vec<Fp2> = Vec::with_capacity(per.len());
            let mut out: Vec<Fp2> = Vec::with_capacity(hi - lo);
            let mut x = shift_c * d.sub.pow(lo as u64);
            for i in lo..hi {
                window.clear();
                for k in 0..d.window {
                    let row = (i + k) % d.t;
                    for col in &cols {
                        window.push(Fp2::from_base(col[row]));
                    }
                }
                periodic_i.clear();
                periodic_i.extend(per.iter().map(|p| Fp2::from_base(p[i])));
                out.push(compose_ext(air, d.g, Fp2::from_base(x), &window, &periodic_i, coeffs));
                x = x * d.sub;
            }
            out
        });
        for (b, part) in parts.into_iter().enumerate() {
            for (k, v) in part.into_iter().enumerate() {
                comp_d[c + d.blowup * (b * BLOCK + k)] = v;
            }
        }
    }
    comp_d
}
