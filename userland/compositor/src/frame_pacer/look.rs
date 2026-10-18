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

//! An output's look: its colour settings (`display::access_rule`) and night
//! light turned into tables once, when either changes, so a frame only looks
//! pixels up. Applied to each composed rectangle before the indicator band,
//! which keeps its true colours.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::display::access_rule::Access;
use crate::display::colour_rule::{matrix, ramps};
use crate::display::srgb_rule::{from_linear, to_linear, LIN_MAX};
use crate::state::damage::Rect;
use crate::sw_blitter::Surface;

/// One in fixed point for the matrix: twelve fraction bits.
pub const ONE: i32 = 1 << 12;

pub struct Look {
    /// The filter's matrix in fixed point, with the sRGB tables it needs.
    pub matrix: Option<[[i32; 3]; 3]>,
    pub lin: [u16; 256],
    pub srgb: Vec<u8>,
    /// Night light's ramps then contrast's, red, green, blue.
    pub ramps: Option<[[u8; 256]; 3]>,
}

/// The look for `a` under night light `night`; None when it changes nothing.
pub fn build(a: &Access, night: Option<&[[u8; 256]; 3]>) -> Option<Box<Look>> {
    let m = matrix(a.filter);
    let ramps = ramps(a, night);
    if m.is_none() && ramps.is_none() {
        return None;
    }
    let fixed = |v: f32| (v * ONE as f32 + if v < 0.0 { -0.5 } else { 0.5 }) as i32;
    let matrix = m.map(|m| m.map(|row| row.map(fixed)));
    let (mut lin, mut srgb) = ([0u16; 256], Vec::new());
    if matrix.is_some() {
        lin = core::array::from_fn(|v| to_linear(v as u8));
        srgb = (0..=LIN_MAX as u16).map(from_linear).collect();
    }
    Some(Box::new(Look { matrix, lin, srgb, ramps }))
}

/// `rect` of `dst` through the look: the matrix, then the ramps.
pub fn apply(dst: Surface, rect: Rect, look: &Look) {
    if let Some(m) = look.matrix {
        super::look_matrix::filter(dst, rect, &m, &look.lin, &look.srgb);
    }
    if let Some(r) = look.ramps.as_ref() {
        super::night_tint::tint(dst, rect, r);
    }
}
