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

//! `fit_size`: the largest size of a ladder at which the run fits the width,
//! whatever order the ladder is listed in, or none when it fits at none.

use nonos_toolkit::widgets::text::{fit_size, fit_size_by, width};

use super::samples::{sizes, tens, RUNS};
use super::serial::serial;

/// The calculator's readout ladder, largest first, then the role sizes.
fn ladder() -> Vec<f32> {
    [56.0, 44.0, 34.0, 26.0, 17.0].into_iter().chain(sizes()).collect()
}

/// What is wrong with `out` as `fit_size(s, max_w, ladder)`, if anything.
fn fault(s: &str, max_w: u32, ladder: &[f32], out: Option<f32>) -> Option<String> {
    let fits = |px: f32| width(s, px) <= max_w;
    if let Some(px) = out.filter(|px| !ladder.contains(px) || !fits(*px)) {
        return Some(format!("{px} is not a size of the ladder that fits"));
    }
    let best = ladder.iter().copied().filter(|px| fits(*px)).reduce(f32::max);
    (best != out).then(|| format!("{out:?} is not the largest fit, {best:?}"))
}

#[test]
fn fit_size_is_the_largest_size_that_fits() {
    let _g = serial();
    let mut fails = Vec::new();
    for ladder in [ladder(), ladder().into_iter().rev().collect()] {
        for s in RUNS {
            for max_w in (0..=width(s, 56.0) + 4).step_by(2) {
                let why = fault(s, max_w, &ladder, fit_size(s, max_w, &ladder));
                fails.extend(why.map(|w| format!("{s:?} in {max_w}: {w}")));
            }
        }
    }
    assert!(fails.is_empty(), "{} cases\n{}", fails.len(), fails.join("\n"));
}

#[test]
fn fit_size_by_takes_the_largest_whatever_the_order() {
    let scaled = |s: &str, px: f32| tens(s) * px as i32 / 10;
    for ladder in [[40.0, 30.0, 20.0], [20.0, 30.0, 40.0], [30.0, 40.0, 20.0]] {
        assert_eq!(fit_size_by("abc", 120, &ladder, scaled), Some(40.0));
        assert_eq!(fit_size_by("abc", 119, &ladder, scaled), Some(30.0));
        assert_eq!(fit_size_by("abc", 60, &ladder, scaled), Some(20.0));
        assert_eq!(fit_size_by("abc", 59, &ladder, scaled), None);
    }
    assert_eq!(fit_size_by("abc", 100, &[], scaled), None);
    assert_eq!(fit_size_by("abc", 100, &[30.0, 30.0, f32::NAN], scaled), Some(30.0));
}

#[test]
fn fit_size_by_measures_every_size_when_widths_do_not_narrow() {
    let clamped = |_: &str, px: f32| if px < 30.0 { 500 } else { px as i32 };
    assert_eq!(fit_size_by("x", 35, &[20.0, 30.0, 40.0], clamped), Some(30.0));
    assert_eq!(fit_size_by("x", 35, &[40.0, 20.0, 30.0], clamped), Some(30.0));
    assert_eq!(fit_size_by("x", 25, &[40.0, 30.0, 20.0], clamped), None);
}
