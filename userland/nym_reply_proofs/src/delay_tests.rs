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

//! The delay a mix holds each packet for: drawn from an exponential
//! distribution in nanoseconds, the unit a mix reads it in.

use crate::exp_delay::{exp_delay_ns, DELAY_CAP_MEANS};

const MEAN: u64 = 15_000_000;

/// A fixed, well spread sequence: splitmix64.
fn draws(n: usize) -> impl Iterator<Item = u64> {
    let mut x = 0x9E37_79B9_7F4A_7C15u64;
    (0..n).map(move |_| {
        x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = x;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    })
}

#[test]
fn the_draw_is_the_inverse_of_the_exponential() {
    // u = 1 is no delay at all; u = 1/2 is the median, mean * ln 2.
    assert_eq!(exp_delay_ns(u64::MAX, MEAN), 0);
    let half = exp_delay_ns(1u64 << 63, MEAN) as f64;
    let median = MEAN as f64 * core::f64::consts::LN_2;
    assert!((half - median).abs() < 2.0, "median {half} against {median}");
    // u = 1/e is exactly the mean.
    let e_inv = ((1.0 / core::f64::consts::E) * (1u64 << 53) as f64) as u64;
    let at_mean = exp_delay_ns((e_inv - 1) << 11, MEAN) as f64;
    assert!((at_mean - MEAN as f64).abs() < 5.0, "{at_mean}");
}

#[test]
fn the_draws_average_the_mean_and_are_not_a_constant() {
    let n = 200_000;
    let all: Vec<u64> = draws(n).map(|r| exp_delay_ns(r, MEAN)).collect();
    let mean = all.iter().sum::<u64>() as f64 / n as f64;
    assert!((mean / MEAN as f64 - 1.0).abs() < 0.01, "sample mean {mean}");
    // An exponential's spread equals its mean.
    let var = all.iter().map(|&d| (d as f64 - mean).powi(2)).sum::<f64>() / n as f64;
    assert!((var.sqrt() / MEAN as f64 - 1.0).abs() < 0.02, "sample deviation {}", var.sqrt());
    // Six in ten fall below the mean: 1 - 1/e.
    let below = all.iter().filter(|&&d| d < MEAN).count() as f64 / n as f64;
    assert!((below - 0.632).abs() < 0.005, "{below}");
}

#[test]
fn a_delay_is_nanoseconds_and_never_runs_past_its_cap() {
    // Milliseconds read as nanoseconds was the old fault: every mix passed
    // the packet straight on. A mean of 15 ms is fifteen million here.
    let typical = draws(1000).map(|r| exp_delay_ns(r, MEAN)).max().unwrap();
    assert!(typical > 1_000_000, "{typical}");
    assert_eq!(exp_delay_ns(0, MEAN), MEAN * DELAY_CAP_MEANS);
    assert!(draws(100_000).all(|r| exp_delay_ns(r, MEAN) <= MEAN * DELAY_CAP_MEANS));
}
