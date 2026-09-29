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

use super::mix_lerp::lerp;
use super::mix_method::method;
use super::rgbaf::Rgbaf;
use crate::browser::css::calc::split_top::{items, words};

/// color-mix(in <space> [<way> hue], <color> [<p>%], <color> [<p>%]).
/// Missing percentages complete each other to 100%; a sum under 100%
/// scales the result's alpha, a sum over it is normalised.
pub(super) fn parse_color_mix(inner: &str) -> Option<u32> {
    let mut parts = items(inner);
    let (head, a, b) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    let (space, way) = method(head)?;
    let (c1, p1) = mix_arg(a)?;
    let (c2, p2) = mix_arg(b)?;
    let (p1, p2) = match (p1, p2) {
        (None, None) => (50.0, 50.0),
        (Some(p), None) => (p, 100.0 - p),
        (None, Some(p)) => (100.0 - p, p),
        (Some(x), Some(y)) => (x, y),
    };
    let sum = p1 + p2;
    if !(sum > 0.0) {
        return None;
    }
    let mut out = lerp(space, way, c1, c2, p2 / sum);
    out.a *= (sum / 100.0).min(1.0);
    Some(out.to_argb())
}

/// One `<color> [<percentage>]` argument, in either order; the
/// percentage must lie in 0..=100.
fn mix_arg(arg: &str) -> Option<(Rgbaf, Option<f64>)> {
    let (mut color, mut pct) = (None, None);
    for w in words(arg) {
        match w.strip_suffix('%').map(|p| p.trim().parse::<f64>()) {
            Some(Ok(p)) if pct.is_none() && (0.0..=100.0).contains(&p) => pct = Some(p),
            Some(_) => return None,
            None if color.is_none() => color = Some(super::parse_color(w)?),
            None => return None,
        }
    }
    Some((Rgbaf::from_argb(color?), pct))
}
