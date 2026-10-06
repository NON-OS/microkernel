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

use super::affine::Affine;

/// A gradient ready to shade device pixels: the map from device space to
/// the gradient's own space, its geometry there (x1 y1 x2 y2, or cx cy r
/// fx fy), how it spreads past its ends, and 256 colours along it.
pub(super) struct GradPaint {
    pub inv: Affine,
    pub radial: bool,
    pub geom: [f32; 5],
    /// 0 pad, 1 reflect, 2 repeat.
    pub spread: u8,
    pub lut: [u32; 256],
}

impl GradPaint {
    /// The colour at device point (x, y).
    pub fn at(&self, x: f32, y: f32) -> u32 {
        let p = self.inv.apply([x, y]);
        let t = if self.radial { self.radial_t(p) } else { self.linear_t(p) };
        let t = match self.spread {
            1 => {
                let m = t - 2.0 * floor(t / 2.0);
                if m > 1.0 {
                    2.0 - m
                } else {
                    m
                }
            }
            2 => t - floor(t),
            _ => t,
        };
        self.lut[(t.clamp(0.0, 1.0) * 255.0 + 0.5) as usize]
    }
}

fn floor(v: f32) -> f32 {
    let t = v as i64 as f32;
    if t > v {
        t - 1.0
    } else {
        t
    }
}
