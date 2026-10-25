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

//! Each decoder beside the test's own statement of what its op takes,
//! written apart from the compositor's constants.

use super::ops_other::{set_vcp, vcp_model};
use crate::requests::toggles;

/// The op's name, its body length, a body's words it takes, its decoder,
/// and what a body of that length must yield from its words.
pub type Op = (&'static str, usize, &'static [u32], Decode, Model);
pub type Decode = fn(&[u8]) -> Option<Vec<u64>>;
pub type Model = fn(&[u32]) -> Option<Vec<u64>>;

pub fn some(xs: &[u32]) -> Option<Vec<u64>> {
    Some(xs.iter().map(|&x| x as u64).collect())
}

pub fn b(x: bool) -> u32 {
    x as u32
}

/// A word that holds only 0 or 1, with `n` words read in all.
fn yes_no(w: &[u32], at: usize, n: usize) -> Option<Vec<u64>> {
    (w[at] <= 1).then(|| some(&w[..n]))?
}

fn flag(d: fn(&[u8]) -> Option<bool>, x: &[u8]) -> Option<Vec<u64>> {
    some(&[b(d(x)?)])
}
fn indicator(x: &[u8]) -> Option<Vec<u64>> {
    toggles::indicator(x).map(|(k, on)| some(&[k, b(on)]))?
}

/// Off, or no warmer than 1000 K.
fn night(w: &[u32]) -> Option<Vec<u64>> {
    (w[0] == 0 || w[0] >= 1000).then(|| some(&w[..1]))?
}

/// Following the panel reads as u64::MAX, apart from any scale.
fn scale(x: &[u8]) -> Option<Vec<u64>> {
    toggles::output_scale(x).map(|(o, q)| vec![o as u64, q.map_or(u64::MAX, u64::from)])
}

/// 0 follows the panel; else 1, 1.25, 1.5, 2, 3 or 4 in quarters.
fn scaled(w: &[u32]) -> Option<Vec<u64>> {
    let q = if w[1] == 0 { u64::MAX } else { w[1].into() };
    [0, 4, 5, 6, 8, 12, 16].contains(&w[1]).then(|| vec![w[0].into(), q])
}

pub fn settings() -> Vec<Op> {
    vec![
        ("lid", 4, &[1], |x| flag(toggles::lid, x), |w| yes_no(w, 0, 1)),
        ("brightness_step", 4, &[1], |x| flag(toggles::brightness_step, x), |w| yes_no(w, 0, 1)),
        ("output_layout", 4, &[0], |x| flag(toggles::output_layout, x), |w| yes_no(w, 0, 1)),
        ("indicator", 8, &[1, 1], indicator, |w| yes_no(w, 1, 2)),
        ("night_light", 4, &[3400], |x| toggles::night_light(x).map(|k| some(&[k]))?, night),
        ("output_scale", 8, &[0, 6], scale, scaled),
        ("set_vcp", 12, &[0, 0x60, 0x0F], set_vcp, vcp_model),
    ]
}
