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
use super::super::super::poly::batch_inv;
use super::compose::BLOCK;
use super::coset::extend;
use super::setup::Domain;
use alloc::vec::Vec;

/// The DEEP polynomial over the domain: every trace quotient against the
/// claimed frame, plus the composition quotient, batched under the DEEP
/// coefficients. Coset-walked like the composition, and the trace columns are
/// extended a second time here rather than kept, because the extension is
/// cheaper than holding width times the domain in memory ever was.
#[allow(clippy::too_many_arguments)]
pub(in crate::air) fn over_domain(
    d: &Domain,
    trace: &[Vec<Fp>],
    comp_d: &[Fp2],
    ood_frame: &[Fp2],
    comp_z: Fp2,
    z: Fp2,
    deep_coeffs: &[Fp2],
) -> Vec<Fp2> {
    // z * g^k once per window row, not once per point.
    let zks: Vec<Fp2> = (0..d.window).map(|k| z * Fp2::from_base(d.g.pow(k as u64))).collect();
    let e = deep_coeffs[d.width * d.window];

    let mut deep_d = alloc::vec![Fp2::ZERO; d.n];
    for c in 0..d.blowup {
        let cols = extend(trace, d, c);
        let shift_c = d.coset_shift(c);
        let blocks = d.t.div_ceil(BLOCK);
        let parts = crate::par::map_index(blocks, |b| {
            let (lo, hi) = (b * BLOCK, ((b + 1) * BLOCK).min(d.t));
            // Every denominator in the block at once: window + 1 of them per
            // point, one field inversion for all of them together.
            let stride = zks.len() + 1;
            let mut dens: Vec<Fp2> = Vec::with_capacity((hi - lo) * stride);
            let mut x = shift_c * d.sub.pow(lo as u64);
            for _ in lo..hi {
                let xe = Fp2::from_base(x);
                for zk in &zks {
                    dens.push(xe - *zk);
                }
                dens.push(xe - z);
                x = x * d.sub;
            }
            let invs = batch_inv(&dens);

            let mut out: Vec<Fp2> = Vec::with_capacity(hi - lo);
            for i in lo..hi {
                let j = c + d.blowup * i;
                let base = (i - lo) * stride;
                let mut acc = Fp2::ZERO;
                for k in 0..zks.len() {
                    let inv_x_zk = invs[base + k];
                    for (col, column) in cols.iter().enumerate() {
                        let claimed = ood_frame[k * d.width + col];
                        acc = acc
                            + deep_coeffs[k * d.width + col]
                                * ((Fp2::from_base(column[i]) - claimed) * inv_x_zk);
                    }
                }
                acc = acc + e * ((comp_d[j] - comp_z) * invs[base + zks.len()]);
                out.push(acc);
            }
            out
        });
        for (b, part) in parts.into_iter().enumerate() {
            for (k, v) in part.into_iter().enumerate() {
                deep_d[c + d.blowup * (b * BLOCK + k)] = v;
            }
        }
    }
    deep_d
}
