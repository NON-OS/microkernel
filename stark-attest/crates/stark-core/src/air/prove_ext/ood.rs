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
use super::super::super::transcript::Transcript;

/// Draw the out-of-domain point from the extension: off the evaluation coset and
/// off the trace domain, so every DEEP and periodic denominator is invertible in
/// `Fp2`. Both sides run this identically, so they agree on the point.
pub(in crate::air) fn draw_ood_point_ext(
    transcript: &mut Transcript,
    shift: Fp,
    n: usize,
    t: usize,
) -> Fp2 {
    let shift_n = Fp2::from_base(shift.pow(n as u64));
    let mut z = transcript.challenge_fp2();
    while z.pow(n as u64) == shift_n || z.pow(t as u64) == Fp2::ONE {
        z = transcript.challenge_fp2();
    }
    z
}
