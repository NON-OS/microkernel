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

/// The map from viewBox `view` ([x, y, w, h]) onto a `w` x `h` viewport
/// under `preserveAspectRatio` (SVG 1.1 7.8): "none" stretches; otherwise
/// the box scales uniformly to fit inside (meet, the default) or to cover
/// (slice), aligned by xMin/xMid/xMax and YMin/YMid/YMax, xMidYMid when
/// the value does not say.
pub(super) fn fit(par: Option<&str>, view: [f32; 4], w: f32, h: f32) -> Affine {
    let [vx, vy, vw, vh] = view;
    let (sx, sy) = (w / vw, h / vh);
    let par = par.unwrap_or("").trim();
    let mut words = par.split_ascii_whitespace().filter(|w| *w != "defer");
    let align = words.next().unwrap_or("xMidYMid");
    if align == "none" {
        return Affine::scale(sx, sy).then(&Affine::translate(-vx, -vy));
    }
    let s = if words.next() == Some("slice") { sx.max(sy) } else { sx.min(sy) };
    let place = |key: [&str; 2], room: f32| {
        if align.contains(key[0]) {
            0.0
        } else if align.contains(key[1]) {
            room
        } else {
            room / 2.0
        }
    };
    let dx = place(["xMin", "xMax"], w - vw * s);
    let dy = place(["YMin", "YMax"], h - vh * s);
    Affine::translate(dx, dy).then(&Affine::scale(s, s)).then(&Affine::translate(-vx, -vy))
}
