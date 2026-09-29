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

use alloc::vec::Vec;

use super::attr::{attr, style_prop};
use super::color::stop_color;
use super::xml::next_tag;

/// A linearGradient or radialGradient as written: its attribute text,
/// looked up through href when a value is missing, and its own stops as
/// (offset, ARGB) with offsets clamped into order.
pub(super) struct Gradient<'a> {
    pub radial: bool,
    pub attrs: &'a str,
    pub stops: Vec<(f32, u32)>,
}

/// The most stops one gradient keeps.
const MAX_STOPS: usize = 64;

/// Read the gradient whose open tag ends at `body` (its `stop` children
/// follow unless the tag closed itself).
pub(super) fn read_gradient<'a>(
    doc: &'a str,
    radial: bool,
    attrs: &'a str,
    closed: bool,
    body: usize,
) -> Gradient<'a> {
    let mut g = Gradient { radial, attrs, stops: Vec::new() };
    let mut pos = body;
    while !closed {
        let Some((tag, next)) = next_tag(doc, pos) else { break };
        pos = next;
        if tag.closing && tag.name.ends_with("Gradient") {
            break;
        }
        if tag.closing || tag.name != "stop" || g.stops.len() == MAX_STOPS {
            continue;
        }
        let style = attr(tag.attrs, "style").unwrap_or("");
        let prop = |n: &str| style_prop(style, n).or_else(|| attr(tag.attrs, n));
        let at = prop("offset").and_then(fraction).unwrap_or(0.0).clamp(0.0, 1.0);
        let at = g.stops.last().map_or(at, |s| at.max(s.0));
        let opacity = prop("stop-opacity").and_then(fraction).unwrap_or(1.0);
        g.stops.push((at, stop_color(prop("stop-color").unwrap_or("black"), opacity)));
    }
    g
}

/// A number, or a percentage as a fraction ("50%" is 0.5).
pub(super) fn fraction(v: &str) -> Option<f32> {
    let t = v.trim();
    let (num, k) = t.strip_suffix('%').map_or((t, 1.0), |n| (n, 0.01));
    let x = num.trim().parse::<f32>().ok()? * k;
    x.is_finite().then_some(x)
}
