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

//! Drawing a mix delay. Pure, so the host proofs hold it to the distribution.

/// The longest single delay drawn, as a multiple of the mean. The tail past it
/// is under one draw in a hundred million, and cutting it keeps one unlucky
/// packet from holding a page for seconds.
pub const DELAY_CAP_MEANS: u64 = 20;

/// An exponentially distributed delay with mean `mean_ns`, from 64 random
/// bits: the inverse of the distribution at a uniform point in (0, 1].
pub fn exp_delay_ns(random: u64, mean_ns: u64) -> u64 {
    // 53 bits is all an f64 holds exactly; adding one keeps the point off
    // zero, where the logarithm has no value.
    let u = ((random >> 11) + 1) as f64 / (1u64 << 53) as f64;
    let ns = -ln(u) * mean_ns as f64;
    let cap = mean_ns.saturating_mul(DELAY_CAP_MEANS);
    if ns >= cap as f64 {
        return cap;
    }
    // Rounded to the nearest nanosecond; never negative, since u <= 1.
    (ns + 0.5) as u64
}

/// Natural logarithm of `x` in (0, 1], without a maths library.
///
/// The exponent is taken off the float's own bits, leaving a mantissa m in
/// [1, 2), and ln(m) = 2 atanh((m - 1) / (m + 1)), whose series converges
/// within a dozen terms for an argument under a third.
fn ln(x: f64) -> f64 {
    const LN_2: f64 = core::f64::consts::LN_2;
    let bits = x.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i64 - 1023;
    let m = f64::from_bits((bits & 0x000f_ffff_ffff_ffff) | (1023u64 << 52));
    let t = (m - 1.0) / (m + 1.0);
    let t2 = t * t;
    let mut term = t;
    let mut sum = 0.0;
    let mut k = 1.0;
    while k < 26.0 {
        sum += term / k;
        term *= t2;
        k += 2.0;
    }
    exponent as f64 * LN_2 + 2.0 * sum
}
