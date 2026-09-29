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

use super::mix_space::Space;
use super::rgbaf::Rgbaf;
use super::trig::wrap_deg;

/// How a polar space walks the hue circle between the two colours.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum HueWay {
    Shorter,
    Longer,
    Increasing,
    Decreasing,
}

/// Mix `c1` and `c2` in `space` with `t` the weight of `c2`: channels are
/// premultiplied by alpha, the hue is not, and a grey takes the other hue.
pub(super) fn lerp(space: Space, way: HueWay, c1: Rgbaf, c2: Rgbaf, t: f64) -> Rgbaf {
    let (mut v1, grey1) = space.coords(c1);
    let (mut v2, grey2) = space.coords(c2);
    let hue = space.hue();
    if let Some(i) = hue {
        if grey1 && !grey2 {
            v1[i] = v2[i];
        } else if grey2 && !grey1 {
            v2[i] = v1[i];
        }
    }
    let (w1, w2) = (c1.a * (1.0 - t), c2.a * t);
    let a = w1 + w2;
    let mut out = [0.0; 3];
    for (k, o) in out.iter_mut().enumerate() {
        *o = if hue == Some(k) {
            hue_lerp(v1[k], v2[k], t, way)
        } else if a > 0.0 {
            (v1[k] * w1 + v2[k] * w2) / a
        } else {
            v1[k] * (1.0 - t) + v2[k] * t
        };
    }
    space.color(out, a)
}

fn hue_lerp(h1: f64, h2: f64, t: f64, way: HueWay) -> f64 {
    let (mut a, mut b) = (wrap_deg(h1), wrap_deg(h2));
    let d = b - a;
    match way {
        HueWay::Shorter if d > 180.0 => a += 360.0,
        HueWay::Shorter if d < -180.0 => b += 360.0,
        HueWay::Longer if d > 0.0 && d < 180.0 => a += 360.0,
        HueWay::Longer if d > -180.0 && d <= 0.0 => b += 360.0,
        HueWay::Increasing if d < 0.0 => b += 360.0,
        HueWay::Decreasing if d > 0.0 => a += 360.0,
        _ => {}
    }
    a + (b - a) * t
}
