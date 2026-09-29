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

/* The float functions colour conversion needs. core carries no libm, so
 * these are series in f64, accurate far past what an 8-bit channel shows. */

const LN2: f64 = core::f64::consts::LN_2;

/// Largest integer not above `x` (for |x| below 2^52).
pub(super) fn floor(x: f64) -> f64 {
    let t = x as i64 as f64;
    t - (t > x) as u8 as f64
}

/// Natural logarithm of a positive finite `x`; anything else gives NaN.
pub(super) fn ln(x: f64) -> f64 {
    if !(x > 0.0 && x.is_finite()) {
        return f64::NAN;
    }
    /* A subnormal is scaled into the normal range first. */
    let (x, bias) =
        if x < f64::MIN_POSITIVE { (x * 18_014_398_509_481_984.0, -54.0) } else { (x, 0.0) };
    let bits = x.to_bits();
    let mut e = ((bits >> 52) & 0x7ff) as f64 - 1023.0 + bias;
    let mut m = f64::from_bits((bits & 0x000f_ffff_ffff_ffff) | 0x3ff0_0000_0000_0000);
    if m > core::f64::consts::SQRT_2 {
        m /= 2.0;
        e += 1.0;
    }
    /* ln(m) = 2 atanh(t) with |t| <= 0.172, so 14 odd terms are exact. */
    let t = (m - 1.0) / (m + 1.0);
    let (t2, mut term, mut sum) = (t * t, t, 0.0);
    for k in 0..14 {
        sum += term / (2 * k + 1) as f64;
        term *= t2;
    }
    2.0 * sum + e * LN2
}

/// e to the power `x`, 0 far below and infinity far above the f64 range.
pub(super) fn exp(x: f64) -> f64 {
    if x.is_nan() {
        return f64::NAN;
    }
    if x > 709.0 {
        return f64::INFINITY;
    }
    if x < -708.0 {
        return 0.0;
    }
    let n = floor(x / LN2 + 0.5);
    let r = x - n * LN2;
    let (mut term, mut sum) = (1.0, 1.0);
    for k in 1..18 {
        term *= r / k as f64;
        sum += term;
    }
    sum * f64::from_bits(((n as i64 + 1023) as u64) << 52)
}
