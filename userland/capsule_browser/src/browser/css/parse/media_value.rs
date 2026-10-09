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

/* The width, height or aspect-ratio a range names, at this viewport. */
pub(super) fn feature(t: &str, w: f32, h: f32) -> Option<f32> {
    Some(match t {
        "width" => w,
        "height" => h,
        "aspect-ratio" => w / h,
        _ => return None,
    })
}

pub(super) fn compare(x: f32, op: &str, y: f32) -> bool {
    match op {
        "<" => x < y,
        "<=" => x <= y,
        ">" => x > y,
        ">=" => x >= y,
        _ => (x - y).abs() < 0.001,
    }
}

/* px, em and rem, em against the 16px initial font size, and a bare 0. */
pub(super) fn length(t: &str) -> Option<f32> {
    let (num, scale) = match t {
        _ if t.ends_with("rem") => (&t[..t.len() - 3], 16.0),
        _ if t.ends_with("em") => (&t[..t.len() - 2], 16.0),
        _ if t.ends_with("px") => (&t[..t.len() - 2], 1.0),
        _ => (t, 0.0),
    };
    let v = num.trim().parse::<f32>().ok()?;
    (scale != 0.0 || v == 0.0).then_some(v * scale)
}

/* A ratio a/b, or a bare number meaning a/1. */
pub(super) fn ratio(t: &str) -> Option<f32> {
    let (a, b) = t.split_once('/').unwrap_or((t, "1"));
    let (a, b) = (a.trim().parse::<f32>().ok()?, b.trim().parse::<f32>().ok()?);
    (b != 0.0).then_some(a / b)
}
