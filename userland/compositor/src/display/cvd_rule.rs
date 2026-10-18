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

//! Colour-vision filters, derived here rather than copied. A dichromat lacks
//! one of the three cone types, so every colour is seen as one on a plane in
//! cone (LMS) space holding black, white and a hue the dichromat sees truly
//! (Brettel, Viénot and Mollon 1997, JOSA A 14; one plane, after Viénot,
//! Brettel and Mollon 1999, Color Research and Application 24; both from
//! memory). The hue is taken as a display primary: blue for protanopes and
//! deuteranopes, which gives the published coefficients exactly (the
//! proofs check it); red, for 660 nm, for tritanopes, which gives a row of
//! its own, since the tabulated tritan row rests on spectral anchors not
//! reproduced here. The lost cone's response is rebuilt from the other two
//! so both anchors come out unchanged; solving for it gives the simulation.

pub use super::m3::{apply, inverse, mul, M3};

/// Linear sRGB to LMS, the matrix those authors give for a standard display
/// and Fidaner, Lin and Ozguven (2005, "Analysis of Color Blindness",
/// Stanford) quote; taken from memory, not checked against either here.
pub const RGB_TO_LMS: M3 =
    [[17.8824, 43.5161, 4.11935], [3.45565, 27.1554, 3.86714], [0.0299566, 0.184309, 1.46709]];

/// LMS seen with cone `lost` missing: its response is a*one + b*other of the
/// remaining two, a and b solved so both `anchors` (linear RGB) are kept.
pub fn projection(lost: usize, anchors: [[f32; 3]; 2]) -> Option<M3> {
    let (p, q) = ((lost + 1) % 3, (lost + 2) % 3);
    let [x, y] = anchors.map(|rgb| apply(&RGB_TO_LMS, rgb));
    let det = x[p] * y[q] - x[q] * y[p];
    if det.abs() < 1e-9 {
        return None;
    }
    let a = (x[lost] * y[q] - x[q] * y[lost]) / det;
    let b = (x[p] * y[lost] - x[lost] * y[p]) / det;
    let mut m = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    m[lost] = [0.0; 3];
    (m[lost][p], m[lost][q]) = (a, b);
    Some(m)
}
