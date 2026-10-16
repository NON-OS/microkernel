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

//! The colour-vision matrices derived from cone space: the protan and deutan
//! coefficients come out as the published ones (Viénot, Brettel and Mollon's
//! method, as Fidaner et al. tabulate it), each simulation keeps its anchors
//! and is a projection, and each correction keeps black, white and grey.

use crate::display::cvd_filter::{correct, simulate, Cvd};
use crate::display::cvd_rule::{apply, mul, projection};

const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

fn near(a: [f32; 3], b: [f32; 3], tol: f32) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < tol)
}

#[test]
fn the_lost_cone_is_rebuilt_as_published() {
    let p = projection(0, [WHITE, [0.0, 0.0, 1.0]]).unwrap();
    assert!((p[0][1] - 2.02344).abs() < 1e-3 && (p[0][2] + 2.52581).abs() < 1e-3, "{p:?}");
    let d = projection(1, [WHITE, [0.0, 0.0, 1.0]]).unwrap();
    assert!((d[1][0] - 0.494207).abs() < 1e-3 && (d[1][2] - 1.24827).abs() < 1e-3, "{d:?}");
    // The tritan row Fidaner et al. tabulate rests on spectral anchors not
    // reproduced here; ours is the plane through white and the red primary,
    // which keeps both (next test) and white's blue cone exactly.
    let t = projection(2, [WHITE, [1.0, 0.0, 0.0]]).unwrap();
    let lms_white = apply(&crate::display::cvd_rule::RGB_TO_LMS, WHITE);
    let s = t[2][0] * lms_white[0] + t[2][1] * lms_white[1];
    assert!((s - lms_white[2]).abs() < 1e-4, "{t:?}");
}

#[test]
fn a_simulation_keeps_its_anchors_and_is_a_projection() {
    for (kind, anchor) in [
        (Cvd::Protan, [0.0, 0.0, 1.0]),
        (Cvd::Deutan, [0.0, 0.0, 1.0]),
        (Cvd::Tritan, [1.0, 0.0, 0.0]),
    ] {
        let s = simulate(kind).unwrap();
        assert!(near(apply(&s, WHITE), WHITE, 1e-4), "{kind:?} keeps white");
        assert!(near(apply(&s, anchor), anchor, 1e-4), "{kind:?} keeps its hue");
        let twice = mul(&s, &s);
        assert!((0..3).all(|i| near(twice[i], s[i], 1e-3)), "{kind:?}: seen once is seen");
    }
}

#[test]
fn a_correction_keeps_the_greys() {
    for kind in [Cvd::Protan, Cvd::Deutan, Cvd::Tritan] {
        let m = correct(kind).unwrap();
        for g in [0.0, 0.18, 0.5, 1.0] {
            assert!(near(apply(&m, [g; 3]), [g; 3], 1e-4), "{kind:?} grey {g}");
        }
    }
}
