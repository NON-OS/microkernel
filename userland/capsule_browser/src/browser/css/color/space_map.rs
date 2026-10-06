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

use super::hwb::hwb_rgb;
use super::lab::{lab_to_linear, linear_to_lab};
use super::mix_space::Space;
use super::oklab::{from_polar, linear_to_oklab, oklab_to_linear};
use super::powers::sqrt;
use super::rgbaf::Rgbaf;
use super::trig::atan2_deg;
use crate::browser::css::hsl_fn::{hsl_rgb, rgb_hsl};

/* Conversions between sRGB and each color-mix() space. */
impl Space {
    /// Coordinates of `c` here, and whether its hue is powerless (grey).
    pub(super) fn coords(self, c: Rgbaf) -> ([f64; 3], bool) {
        let polar = |v: [f64; 3], eps: f64| {
            let ch = sqrt(v[1] * v[1] + v[2] * v[2]);
            ([v[0], ch, atan2_deg(v[2], v[1])], ch < eps)
        };
        let (h, s, l) = rgb_hsl(c.rgb);
        match self {
            Space::Srgb => (c.rgb, false),
            Space::Linear => (c.linear(), false),
            Space::Lab => (linear_to_lab(c.linear()), false),
            Space::Oklab => (linear_to_oklab(c.linear()), false),
            Space::Lch => polar(linear_to_lab(c.linear()), 0.0375),
            Space::Oklch => polar(linear_to_oklab(c.linear()), 1e-4),
            Space::Hsl => ([h.unwrap_or(0.0), s, l], h.is_none()),
            Space::Hwb => {
                let [r, g, b] = c.rgb;
                ([h.unwrap_or(0.0), r.min(g).min(b), 1.0 - r.max(g).max(b)], h.is_none())
            }
        }
    }

    pub(super) fn color(self, v: [f64; 3], a: f64) -> Rgbaf {
        let lin = |l: [f64; 3]| Rgbaf::from_linear(l, a);
        match self {
            Space::Srgb => Rgbaf { rgb: v, a },
            Space::Linear => lin(v),
            Space::Lab => lin(lab_to_linear(v)),
            Space::Oklab => lin(oklab_to_linear(v)),
            Space::Lch => lin(lab_to_linear(from_polar(v[0], v[1], v[2]))),
            Space::Oklch => lin(oklab_to_linear(from_polar(v[0], v[1], v[2]))),
            Space::Hsl => Rgbaf { rgb: hsl_rgb(v[0], v[1], v[2]), a },
            Space::Hwb => Rgbaf { rgb: hwb_rgb(v[0], v[1], v[2]), a },
        }
    }
}
