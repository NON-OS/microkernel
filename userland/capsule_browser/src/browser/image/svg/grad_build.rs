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
use super::defs::Defs;
use super::grad_lut::lut;
use super::grad_paint::GradPaint;
use super::grad_stops::{get, stops_of};
use super::gradient::fraction;
use super::math::sqrt;
use super::transform::parse_transform;

/// Gradient `i` shading an element whose geometry has user-space bounding
/// box `bbox` ([x0, y0, x1, y1]) under user-to-device map `t`, its colours
/// faded by `opacity`. None when there is nothing to paint: no stops, or
/// bounding-box units on a box with no width or height.
pub(super) fn build(
    defs: &Defs,
    i: usize,
    bbox: [f32; 4],
    t: &Affine,
    opacity: f32,
) -> Option<GradPaint> {
    let lut = lut(stops_of(defs, i), opacity)?;
    let user = get(defs, i, "gradientUnits").is_some_and(|u| u.trim() == "userSpaceOnUse");
    let [_, _, vw, vh] = defs.view;
    /* User-space percentages refer to the viewBox: x to its width, y to
     * its height, a radius to its normalized diagonal. */
    let diag = sqrt((vw * vw + vh * vh) / 2.0);
    let coord = |name: &str, default: f32, extent: f32| {
        let v = get(defs, i, name).map(|v| v.trim().trim_end_matches("px"));
        let pct = v.is_some_and(|v| v.ends_with('%'));
        v.and_then(fraction).map_or(default, |x| if user && pct { x * extent } else { x })
    };
    let (w, h, d) = if user { (vw, vh, diag) } else { (1.0, 1.0, 1.0) };
    let radial = defs.grads[i].1.radial;
    let geom = if radial {
        let (cx, cy, r) =
            (coord("cx", 0.5 * w, vw), coord("cy", 0.5 * h, vh), coord("r", 0.5 * d, diag));
        let (fx, fy) = (coord("fx", cx, vw), coord("fy", cy, vh));
        let (dx, dy) = (fx - cx, fy - cy);
        let far = sqrt(dx * dx + dy * dy);
        let k = if far > 0.99 * r && far > 0.0 { 0.99 * r / far } else { 1.0 };
        [cx, cy, r, cx + dx * k, cy + dy * k]
    } else {
        [coord("x1", 0.0, vw), coord("y1", 0.0, vh), coord("x2", w, vw), coord("y2", 0.0, vh), 0.0]
    };
    let unit = match user {
        true => Affine::identity(),
        false if bbox[2] > bbox[0] && bbox[3] > bbox[1] => Affine::translate(bbox[0], bbox[1])
            .then(&Affine::scale(bbox[2] - bbox[0], bbox[3] - bbox[1])),
        false => return None,
    };
    let own = get(defs, i, "gradientTransform").map_or(Affine::identity(), parse_transform);
    let inv = t.then(&unit).then(&own).invert()?;
    let spread = match get(defs, i, "spreadMethod").map(str::trim) {
        Some("reflect") => 1,
        Some("repeat") => 2,
        _ => 0,
    };
    Some(GradPaint { inv, radial, geom, spread, lut })
}
