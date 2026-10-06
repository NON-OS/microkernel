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
use super::super::super::fri::root_of_unity;
use super::super::composition::domain_params_blown;
use super::super::spec::AirExt;

/// The coset shift every NONOS STARK domain uses.
pub(in crate::air) const SHIFT: u64 = 7;

/// The evaluation geometry every pass reads. The domain is `blowup` cosets of
/// the order-`t` subgroup: position `j = c + blowup * i` is the point
/// `shift * omega^c * sub^i`, so a pass that walks one coset at a time touches
/// every point exactly once without ever holding the whole domain.
pub(in crate::air) struct Domain {
    pub t: usize,
    pub n: usize,
    pub width: usize,
    pub window: usize,
    pub blowup: usize,
    pub fri_log_blowup: u32,
    pub g: Fp,
    pub omega: Fp,
    pub shift: Fp,
    /// Generator of the order-`t` subgroup the cosets share: `omega^blowup`.
    pub sub: Fp,
}

impl Domain {
    pub fn of<A: AirExt>(air: &A, extra_blowup_bits: u32) -> Domain {
        let log_t = air.log_trace_len();
        let t = 1usize << log_t;
        let (log_n, fri_log_blowup) = domain_params_blown(air, extra_blowup_bits);
        let n = 1usize << log_n;
        let blowup = 1usize << (log_n - log_t);
        let omega = root_of_unity(log_n);
        Domain {
            t,
            n,
            width: air.trace_width(),
            window: air.window_size(),
            blowup,
            fri_log_blowup,
            g: root_of_unity(log_t),
            omega,
            shift: Fp::from_u64(SHIFT),
            sub: omega.pow(blowup as u64),
        }
    }

    /// Where coset `c` starts: the point at position `j = c`.
    /// The j-th point of the evaluation domain: shift * omega^j.
    pub fn point(&self, j: usize) -> Fp {
        self.shift * self.omega.pow(j as u64)
    }

    pub fn coset_shift(&self, c: usize) -> Fp {
        self.shift * self.omega.pow(c as u64)
    }
}
