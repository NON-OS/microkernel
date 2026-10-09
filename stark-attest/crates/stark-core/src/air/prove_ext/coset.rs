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

use super::super::super::field::Fp;
use super::super::super::poly::{intt, lde_from_coeffs};
use super::setup::Domain;
use alloc::vec::Vec;

/// Row-major trace to per-column coefficient form. The coefficients are the
/// polynomial; every pass extends them onto whichever coset it is walking, so
/// nothing ever holds a column over the full evaluation domain.
pub(in crate::air) fn trace_coeffs(trace: &[Fp], d: &Domain) -> Vec<Vec<Fp>> {
    crate::par::map_index(d.width, |c| {
        let column: Vec<Fp> = (0..d.t).map(|i| trace[i * d.width + c]).collect();
        intt(&column, d.g)
    })
}

pub(in crate::air) fn periodic_coeffs(cols: &[Vec<Fp>], d: &Domain) -> Vec<Vec<Fp>> {
    crate::par::map_slice(cols, |col| intt(col, d.g))
}

/// Every column evaluated over coset `c`: row `i` of the result is position
/// `c + blowup * i` of the full domain.
pub(in crate::air) fn extend(coeffs: &[Vec<Fp>], d: &Domain, c: usize) -> Vec<Vec<Fp>> {
    let shift_c = d.coset_shift(c);
    crate::par::map_slice(coeffs, |cf| lde_from_coeffs(cf, shift_c, d.sub, d.t))
}
