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

//! The out-of-domain composition check for an arbitrary inner AIR. Where
//! `ComposeCheck` hand-arithmetizes the join-split's three transitions, this
//! recomputes *any* inner AIR's transition from the out-of-domain frame by
//! evaluating the inner AIR's own constraint code at `Ext2<F>`, the inner
//! extension carried as base-field pairs. It reproduces `compose_ext`
//! element for element: the vanishing tower and factor `E = (z - g^(t-1)) *
//! (z^t - 1)^-1`, the `num_transition` recomputed transition values, the boundary
//! quotients, and their batched sum against the claimed `comp_z`. Every count is
//! read from the inner AIR, so the same gadget serves a 3-constraint join-split or
//! a 62-constraint step AIR. Window size two only, matching every money-grade
//! inner in use; a wider window would need the exempt product built incrementally.

use super::super::field::{Ext2, Felt, Fp, Fp2};
use super::compose_check::ComposeBoundary;
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

/// An inner AIR whose transition can be evaluated over any field, so a recursive
/// verifier can recompute it over the tower `Ext2<F>`. An AIR that writes its
/// transition once over `Felt` (the object-safe pattern) satisfies this by
/// forwarding to that one definition; the generic method keeps it off the
/// object-safe `AirExt`, so `ComposeCheckGen` is monomorphized on the concrete
/// inner rather than boxed.
pub trait GenericTransition {
    fn transition_gen<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F>;
}

/// The trace slot layout, all counts derived from the inner AIR. A slot is one
/// `Ext2` value, two base cells.
struct Slots {
    w: usize,  // frame length = inner window_size * trace_width
    p: usize,  // inner periodic count
    nt: usize, // inner num_transition
    b: usize,  // inner boundary count
    k: usize,  // tower height = inner log_trace_len, so z^t = z^(2^k)
}

impl Slots {
    fn frame(&self, i: usize) -> usize {
        i
    }
    fn periodic(&self, i: usize) -> usize {
        self.w + i
    }
    fn z(&self) -> usize {
        self.w + self.p
    }
    fn coeff(&self, i: usize) -> usize {
        self.w + self.p + 1 + i
    }
    fn z_h_inv(&self) -> usize {
        self.w + self.p + 1 + self.nt + self.b
    }
    fn e(&self) -> usize {
        self.z_h_inv() + 1
    }
    fn out(&self, i: usize) -> usize {
        self.e() + 1 + i
    }
    fn comp_z(&self) -> usize {
        self.out(self.nt)
    }
    fn quot(&self, j: usize) -> usize {
        self.comp_z() + 1 + j
    }
    fn tower(&self, k: usize) -> usize {
        self.quot(self.b) + k
    }
    fn total(&self) -> usize {
        self.tower(self.k)
    }
    /// Ext2 transition constraints: tower + z_h_inv + E + out + boundary + comp_z.
    fn num_constraints(&self) -> usize {
        self.k + 1 + 1 + self.nt + self.b + 1
    }
}

pub struct ComposeCheckGen<A> {
    air: A,
    frame: Vec<Fp2>,
    periodic: Vec<Fp2>,
    coeffs: Vec<Fp2>,
    z: Fp2,
    comp_z: Fp2,
    g_tm1: Fp,
    t: u64,
    boundaries: Vec<ComposeBoundary>,
    slots: Slots,
}

impl<A: AirExt + GenericTransition> ComposeCheckGen<A> {
    /// Build the witness-form check from the inner AIR, its out-of-domain frame,
    /// the periodic values and batching coefficients at `z`, the point, the claimed
    /// composition, and the inner trace-domain generator `g`. The public statement
    /// rides the trace and is bound by the assembly, so the AIR is
    /// instance-independent.
    #[allow(clippy::too_many_arguments)]
    pub fn new_witness(
        air: A,
        frame: Vec<Fp2>,
        periodic: Vec<Fp2>,
        coeffs: Vec<Fp2>,
        z: Fp2,
        comp_z: Fp2,
        g: Fp,
    ) -> ComposeCheckGen<A> {
        assert_eq!(air.window_size(), 2, "compose gadget supports window size two");
        let log_t = air.log_trace_len();
        let t = 1u64 << log_t;
        let slots = Slots {
            w: air.window_size() * air.trace_width(),
            p: air.periodic_columns().len(),
            nt: air.num_transition(),
            b: air.boundary().len(),
            k: log_t as usize,
        };
        let boundaries = air
            .boundary()
            .iter()
            .map(|(col, row, e)| ComposeBoundary {
                col: *col,
                g_row: g.pow(*row as u64),
                expected: *e,
            })
            .collect();
        ComposeCheckGen {
            air,
            frame,
            periodic,
            coeffs,
            z,
            comp_z,
            g_tm1: g.pow(t - 1),
            t,
            boundaries,
            slots,
        }
    }

    // The cell-column accessors: this region is the single source of truth for
    // its own slot layout, so a recursive assembly binds its frame, periodic,
    // point, coefficient, and composition cells to their sources without
    // duplicating the slot formula (which would drift from `trace`). Each returns
    // the base column of the `c0` lane; the `c1` lane is the next column.

    /// The base column of frame value `i`.
    pub fn frame_col(&self, i: usize) -> usize {
        2 * self.slots.frame(i)
    }
    /// The base column of periodic value `j` at the point.
    pub fn periodic_col(&self, j: usize) -> usize {
        2 * self.slots.periodic(j)
    }
    /// The base column of the out-of-domain point `z`.
    pub fn z_col(&self) -> usize {
        2 * self.slots.z()
    }
    /// The base column of batching coefficient `i`.
    pub fn coeff_col(&self, i: usize) -> usize {
        2 * self.slots.coeff(i)
    }
    /// The base column of the claimed composition value.
    pub fn comp_z_col(&self) -> usize {
        2 * self.slots.comp_z()
    }
    /// The frame length, so a binding loop sizes to the inner AIR.
    pub fn frame_len(&self) -> usize {
        self.slots.w
    }
    /// The batching-coefficient count (transitions plus boundaries).
    pub fn num_coeff(&self) -> usize {
        self.slots.nt + self.slots.b
    }

    /// The witness: row 0 holds the frame, the periodic and coefficient values, the
    /// point, the witnessed intermediates (vanishing tower, `z_h_inv`, `E`, the
    /// recomputed transitions, the boundary quotients) and the claimed composition;
    /// row 1 is inert padding.
    pub fn trace(&self) -> Vec<Fp> {
        let s = &self.slots;
        let width = self.trace_width();
        let mut tr = alloc::vec![Fp::ZERO; 2 * width];
        let mut put = |slot: usize, v: Fp2| {
            tr[2 * slot] = v.c0;
            tr[2 * slot + 1] = v.c1;
        };

        for (i, v) in self.frame.iter().enumerate() {
            put(s.frame(i), *v);
        }
        for (i, v) in self.periodic.iter().enumerate() {
            put(s.periodic(i), *v);
        }
        put(s.z(), self.z);
        for (i, v) in self.coeffs.iter().enumerate() {
            put(s.coeff(i), *v);
        }

        let z = self.z;
        let z_h_inv = (z.pow(self.t) - Fp2::ONE).inv();
        put(s.z_h_inv(), z_h_inv);
        let e = (z - Fp2::from_base(self.g_tm1)) * z_h_inv;
        put(s.e(), e);

        let out = self.air.transition_ext(&self.frame, &self.periodic);
        for (i, v) in out.iter().enumerate() {
            put(s.out(i), *v);
        }
        put(s.comp_z(), self.comp_z);

        for (j, b) in self.boundaries.iter().enumerate() {
            let q = (self.frame[b.col] - Fp2::from_base(b.expected))
                * (z - Fp2::from_base(b.g_row)).inv();
            put(s.quot(j), q);
        }

        let mut zp = z;
        for k in 0..s.k {
            zp = zp.square();
            put(s.tower(k), zp);
        }
        tr
    }

    fn transition_impl<F: Felt>(&self, window: &[F], _periodic: &[F]) -> Vec<F> {
        let s = &self.slots;
        let rd = |slot: usize| -> Ext2<F> { Ext2::new(window[2 * slot], window[2 * slot + 1]) };
        let base = |v: Fp| -> Ext2<F> { Ext2::from_base(v) };
        let one = Ext2::<F>::ONE;
        let mut res: Vec<Ext2<F>> = Vec::with_capacity(s.num_constraints());

        // The vanishing power tower: tower[k] = z^(2^(k+1)), so tower[k-1] = z^t.
        let z = rd(s.z());
        let mut prev = z;
        for k in 0..s.k {
            let zp = rd(s.tower(k));
            res.push(zp - prev * prev);
            prev = zp;
        }
        // z_h_inv * (z^t - 1) = 1.
        let z_h_inv = rd(s.z_h_inv());
        let zt = rd(s.tower(s.k - 1));
        res.push(z_h_inv * (zt - one) - one);
        // E = (z - g^(t-1)) * z_h_inv.
        let e = rd(s.e());
        res.push(e - (z - base(self.g_tm1)) * z_h_inv);

        // Recompute every inner transition from the frame over the tower Ext2<F>,
        // and pin each witnessed value to it. This is where an arbitrary inner AIR
        // is arithmetized: its own constraint code, evaluated at Ext2<F>.
        let frame: Vec<Ext2<F>> = (0..s.w).map(|i| rd(s.frame(i))).collect();
        let per: Vec<Ext2<F>> = (0..s.p).map(|i| rd(s.periodic(i))).collect();
        let recomputed = self.air.transition_gen::<Ext2<F>>(&frame, &per);
        for i in 0..s.nt {
            res.push(rd(s.out(i)) - recomputed[i]);
        }

        // Boundary quotients: q * (z - g^row) = frame[col] - expected.
        for (j, b) in self.boundaries.iter().enumerate() {
            let q = rd(s.quot(j));
            let denom = z - base(b.g_row);
            let numer = rd(s.frame(b.col)) - base(b.expected);
            res.push(q * denom - numer);
        }

        // comp_z = sum coeff_i * out_i * E over the transitions, plus the batched
        // boundary quotients under their coefficients.
        let mut acc = Ext2::<F>::ZERO;
        for i in 0..s.nt {
            acc = acc + rd(s.coeff(i)) * rd(s.out(i)) * e;
        }
        for j in 0..s.b {
            acc = acc + rd(s.coeff(s.nt + j)) * rd(s.quot(j));
        }
        res.push(rd(s.comp_z()) - acc);

        let mut flat = Vec::with_capacity(res.len() * 2);
        for v in res {
            flat.push(v.c0);
            flat.push(v.c1);
        }
        flat
    }
}

impl<A: AirExt + GenericTransition> AirExt for ComposeCheckGen<A> {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl<A: AirExt + GenericTransition> Air for ComposeCheckGen<A> {
    fn log_trace_len(&self) -> u32 {
        1
    }

    fn trace_width(&self) -> usize {
        2 * self.slots.total()
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // The recompute constraint carries the inner transition's degree in the
        // frame variables; the comp_z batching is degree three. Take the larger.
        self.air.constraint_degree().max(3)
    }

    fn num_transition(&self) -> usize {
        2 * self.slots.num_constraints()
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // Witness form: nothing pinned; the assembly binds the statement cells to
        // their sources (frame to the DEEP claims, coefficients and point to the
        // transcript, comp_z to the DEEP check).
        Vec::new()
    }
}
