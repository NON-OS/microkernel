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

//! The load average's arithmetic (L56): an exponentially weighted average of
//! the runnable count, sampled every five seconds, over 1, 5 and 15 minute
//! windows. Each sample folds in as avg' = avg * w + now * (1 - w), with
//! w = e^(-sample / window), the discrete form of a first-order decay whose
//! time constant is the window. Pure, so kernel_proofs checks the weights
//! against that definition.

/// Load is held with 11 fractional bits, 2048 reading as 1.00: the ABI's
/// `load_avg_fixed` is in this scale, and 1/2048 is far finer than the two
/// decimals /proc/loadavg prints.
pub(crate) const SCALE_BITS: u32 = 11;
pub(crate) const ONE: u64 = 1 << SCALE_BITS;
/// Seconds between samples; the clock's tick count for a sample agrees.
pub(crate) const SAMPLE_S: u64 = 5;
pub(crate) const WINDOWS_S: [u64; 3] = [60, 300, 900];

/// `ONE * e^(-SAMPLE_S / window_s)` to the nearest unit, summed as a power
/// series in 40-bit fixed point: the exponent is under 0.1, so twelve terms
/// leave an error far below one unit of the result.
pub(crate) const fn weight(window_s: u64) -> u64 {
    const BITS: u32 = 40;
    let mut term: i128 = 1 << BITS;
    let mut sum = term;
    let mut k: i128 = 1;
    while k <= 12 {
        term = -term * SAMPLE_S as i128 / (window_s as i128 * k);
        sum += term;
        k += 1;
    }
    let shift = BITS - SCALE_BITS;
    ((sum + (1 << (shift - 1))) >> shift) as u64
}

pub(crate) const WEIGHTS: [u64; 3] =
    [weight(WINDOWS_S[0]), weight(WINDOWS_S[1]), weight(WINDOWS_S[2])];

/// One sample `now`, a runnable count in the same scale, folded into `avg`,
/// rounded toward the sample: a steady load reads back exactly, and a rising
/// or falling one reaches the sample. Rounded to nearest, an average stuck
/// short of it: idle, the 15-minute one held at 93/2048 and printed 0.05.
pub(crate) const fn fold(avg: u64, w: u64, now: u64) -> u64 {
    let sum = avg * w + now * (ONE - w);
    if now >= avg {
        (sum + ONE - 1) >> SCALE_BITS
    } else {
        sum >> SCALE_BITS
    }
}
