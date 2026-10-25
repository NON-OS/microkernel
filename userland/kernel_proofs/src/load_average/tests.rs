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

use super::loadavg_math::{fold, weight, ONE, SAMPLE_S, WEIGHTS, WINDOWS_S};

#[test]
fn each_weight_is_the_decay_over_one_sample_to_the_nearest_unit() {
    for (w, window) in WEIGHTS.iter().zip(WINDOWS_S) {
        let exact = ONE as f64 * (-(SAMPLE_S as f64) / window as f64).exp();
        assert!((*w as f64 - exact).abs() <= 0.5, "{window}: {w} against {exact}");
    }
    // A window from one sample to an hour stays inside the scale.
    for window in [SAMPLE_S, 7, 3_600] {
        let w = weight(window);
        assert!(w > 0 && w < ONE, "{window}: {w}");
    }
}

#[test]
fn every_sample_moves_the_average_toward_it_and_never_past() {
    for w in WEIGHTS {
        for (avg, now) in [(0, 1), (1, 0), (ONE, 3 * ONE), (3 * ONE, ONE), (93, 0), (5, 6)] {
            let next = fold(avg, w, now);
            assert_ne!(next, avg, "{avg} {now}");
            assert!(if now > avg { next <= now } else { next >= now }, "{avg} {now}");
        }
    }
}

#[test]
fn a_steady_whole_load_reads_back_exactly() {
    for n in [0u64, 1, 2, 7, 64, 1_000] {
        for w in WEIGHTS {
            assert_eq!(fold(n * ONE, w, n * ONE), n * ONE, "{n} {w}");
        }
    }
}

#[test]
fn a_load_that_starts_is_followed_and_one_that_stops_fades() {
    for (w, window) in WEIGHTS.iter().zip(WINDOWS_S) {
        let mut avg = 0;
        // Five windows of a load of one bring it within 1% of one.
        for _ in 0..5 * window / SAMPLE_S {
            avg = fold(avg, *w, ONE);
        }
        assert!((ONE * 99 / 100..=ONE).contains(&avg), "{window}: {avg}");
        // Ten windows idle leave nothing at all.
        for _ in 0..10 * window / SAMPLE_S {
            avg = fold(avg, *w, 0);
        }
        assert_eq!(avg, 0, "{window}");
    }
}
