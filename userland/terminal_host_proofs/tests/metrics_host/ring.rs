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

//! The sparkline ring: order on read-back and clamping on push.

use crate::rail::ring::{SparkRing, SPARK_SAMPLES};

fn chronological(r: &SparkRing) -> Vec<u8> {
    let d = r.slice();
    let n = d.len();
    (0..n).map(|i| d[(r.start() + i) % n]).collect()
}

#[test]
fn a_partial_window_reads_back_in_order() {
    let mut r = SparkRing::new();
    for v in [3u8, 9, 27] {
        r.push(v);
    }
    assert_eq!(r.len(), 3);
    assert_eq!(chronological(&r), vec![3, 9, 27]);
}

#[test]
fn a_wrapped_window_reads_back_in_order() {
    let mut r = SparkRing::new();
    for i in 0..(SPARK_SAMPLES + 5) {
        r.push((i % 101) as u8);
    }
    assert_eq!(r.len(), SPARK_SAMPLES);
    let got = chronological(&r);
    let first = (SPARK_SAMPLES + 5 - SPARK_SAMPLES) % 101;
    assert_eq!(got[0] as usize, first);
    assert_eq!(got[SPARK_SAMPLES - 1] as usize, (SPARK_SAMPLES + 4) % 101);
    for w in got.windows(2) {
        assert_eq!(w[1], w[0] + 1);
    }
}

#[test]
fn pushed_values_are_clamped_to_a_percentage() {
    let mut r = SparkRing::new();
    r.push(240);
    assert_eq!(chronological(&r), vec![100]);
}
