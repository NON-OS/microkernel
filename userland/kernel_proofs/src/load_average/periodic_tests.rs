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

use super::loadavg_math::{fold, ONE, SAMPLE_S, WEIGHTS, WINDOWS_S};

#[test]
fn a_periodic_load_settles_on_its_mean_with_the_kernels_extra_bits() {
    // The kernel holds averages 16 bits finer than it publishes (L63).
    let (extra, hours) = (16u32, 4 * 3600 / SAMPLE_S as usize);
    for pattern in [&[1u64, 0][..], &[1, 0, 0], &[1, 0, 0, 0]] {
        for (w, window) in WEIGHTS.iter().zip(WINDOWS_S) {
            let (mut avg, mut exact) = (0u64, 0f64);
            let decay = (-(SAMPLE_S as f64) / window as f64).exp();
            for i in 0..hours {
                let now = pattern[i % pattern.len()];
                avg = fold(avg, *w, (now * ONE) << extra);
                exact = exact * decay + now as f64 * ONE as f64 * (1.0 - decay);
            }
            let published = (avg + (1 << (extra - 1))) >> extra;
            let off = (published as f64 - exact).abs();
            assert!(off <= 4.0, "{pattern:?} {window}: {published} against {exact}");
        }
    }
}
