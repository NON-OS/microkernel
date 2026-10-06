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
use super::super::prove_ext::{batch_inv, extend, Domain, BLOCK};
use alloc::vec::Vec;

/// The preprocessed DEEP polynomial: the trace and composition quotients of the
/// plain form, plus one quotient per periodic column against its claimed value
/// at z, which is what lets the verifier hold the periodic root as a constant
/// instead of recomputing the schedule. Coset-walked; both column families are
/// extended here rather than held.
#[allow(clippy::too_many_arguments)]
pub(super) fn over_domain(
    d: &Domain,
    trace: &[Vec<Fp>],
    periodic: &[Vec<Fp>],
    comp_d: &[Fp2],
    ood_frame: &[Fp2],
    periodic_z: &[Fp2],
    comp_z: Fp2,
    z: Fp2,
    deep_coeffs: &[Fp2],
) -> Vec<Fp2> {
    let zks: Vec<Fp2> = (0..d.window).map(|k| z * Fp2::from_base(d.g.pow(k as u64))).collect();
    let e = deep_coeffs[d.width * d.window];

    let mut deep_d = alloc::vec![Fp2::ZERO; d.n];
    for c in 0..d.blowup {
        let cols = extend(trace, d, c);
        let per = extend(periodic, d, c);
        let shift_c = d.coset_shift(c);
        let blocks = d.t.div_ceil(BLOCK);
        let parts = crate::par::map_index(blocks, |b| {
            let (lo, hi) = (b * BLOCK, ((b + 1) * BLOCK).min(d.t));
            // Every denominator in the block at once; the (x - z) inverse is
            // shared by the composition and every periodic quotient.
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
                let inv_x_z = invs[base + zks.len()];
                acc = acc + e * ((comp_d[j] - comp_z) * inv_x_z);
                for (pi, pd) in per.iter().enumerate() {
                    let pc = deep_coeffs[d.width * d.window + 1 + pi];
                    acc = acc + pc * ((Fp2::from_base(pd[i]) - periodic_z[pi]) * inv_x_z);
                }
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
