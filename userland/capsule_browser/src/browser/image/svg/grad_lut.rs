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

/// 256 colours along the stops, interpolated with premultiplied alpha and
/// faded by `opacity`; None for no stops, which paints nothing.
pub(super) fn lut(stops: &[(f32, u32)], opacity: f32) -> Option<[u32; 256]> {
    let (first, last) = (*stops.first()?, *stops.last()?);
    let premul = |c: u32| {
        let a = (c >> 24) as f32 / 255.0;
        [
            (c >> 24) as f32,
            ((c >> 16) & 255) as f32 * a,
            ((c >> 8) & 255) as f32 * a,
            (c & 255) as f32 * a,
        ]
    };
    let mut out = [0u32; 256];
    for (i, slot) in out.iter_mut().enumerate() {
        let t = i as f32 / 255.0;
        let k = stops.iter().position(|s| s.0 > t).unwrap_or(stops.len());
        let (lo, hi) = match k {
            0 => (first, first),
            k if k == stops.len() => (last, last),
            k => (stops[k - 1], stops[k]),
        };
        let f = if hi.0 > lo.0 { (t - lo.0) / (hi.0 - lo.0) } else { 0.0 };
        let (a, b) = (premul(lo.1), premul(hi.1));
        let c: [f32; 4] = core::array::from_fn(|j| a[j] + (b[j] - a[j]) * f);
        let alpha = c[0] * opacity.clamp(0.0, 1.0);
        let un = |v: f32| if c[0] > 0.0 { (v * 255.0 / c[0] + 0.5).min(255.0) as u32 } else { 0 };
        *slot = ((alpha + 0.5) as u32).min(255) << 24 | un(c[1]) << 16 | un(c[2]) << 8 | un(c[3]);
    }
    Some(out)
}
