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

extern crate alloc;

use super::super::gf::GF2m;
use super::super::{MCELIECE_K, MCELIECE_M, MCELIECE_N, MCELIECE_T};
use super::poly::poly_eval;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) fn compute_parity_check_matrix(goppa: &[u16], support: &[u16]) -> Vec<Vec<u8>> {
    let r = MCELIECE_N - MCELIECE_K;

    let mut h = vec![vec![0u8; MCELIECE_N]; r];

    for j in 0..MCELIECE_N {
        let alpha = support[j];
        let g_alpha = poly_eval(goppa, alpha);

        if g_alpha == 0 {
            continue;
        }

        let g_alpha_inv = GF2m::inv(g_alpha);

        let mut val = g_alpha_inv;
        for i in 0..MCELIECE_T {
            for b in 0..MCELIECE_M {
                let row = i * MCELIECE_M + b;
                if row < r {
                    h[row][j] = ((val >> b) & 1) as u8;
                }
            }
            val = GF2m::mul(val, alpha);
        }
    }

    h
}

pub(crate) fn to_systematic_form(h: &[Vec<u8>]) -> Option<Vec<u8>> {
    let r = MCELIECE_N - MCELIECE_K;
    let mut matrix = h.to_vec();

    for i in 0..r {
        let pivot_col = (i..MCELIECE_N).find(|&j| matrix[i][j] == 1)?;

        if pivot_col != i {
            for row in &mut matrix {
                row.swap(i, pivot_col);
            }
        }

        let pivot_row = matrix[i].clone();
        for (j, row) in matrix.iter_mut().enumerate().take(r) {
            if j != i && row[i] == 1 {
                for (x, &p) in row.iter_mut().zip(&pivot_row).take(MCELIECE_N) {
                    *x ^= p;
                }
            }
        }
    }

    let mut t_matrix = Vec::with_capacity(r * MCELIECE_K / 8);

    for row_bits in matrix.iter().take(r) {
        let mut byte = 0u8;
        let mut bit_pos = 0;

        for &bit in &row_bits[r..MCELIECE_N] {
            byte |= bit << bit_pos;
            bit_pos += 1;

            if bit_pos == 8 {
                t_matrix.push(byte);
                byte = 0;
                bit_pos = 0;
            }
        }

        if bit_pos > 0 {
            t_matrix.push(byte);
        }
    }

    Some(t_matrix)
}
