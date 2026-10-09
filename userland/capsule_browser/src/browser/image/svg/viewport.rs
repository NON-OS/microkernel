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

use super::attr::attr;
use super::num::{abs_len, num_list};
use super::xml::next_tag;

/// The root `<svg>`: its attribute text, where its content starts, the
/// user-space rectangle drawn (viewBox, [minx, miny, w, h]) and its
/// natural size in px.
pub(super) struct Viewport<'a> {
    pub root_attrs: &'a str,
    pub self_closing: bool,
    pub body_at: usize,
    pub view: [f32; 4],
    pub natural: (f32, f32),
}

/// Read the root element the way an `<img>` sizes it: width and height
/// in absolute units; a missing side follows the viewBox ratio; with
/// neither, the 300 x 150 default object size contained to that ratio;
/// with no viewBox either, 300 x 150. Percentages set no size.
pub(super) fn viewport(doc: &str) -> Option<Viewport<'_>> {
    let mut pos = 0;
    let (root, body_at) = loop {
        let (t, n) = next_tag(doc, pos)?;
        if !t.closing && t.name == "svg" {
            break (t, n);
        }
        pos = n;
    };
    let vb = attr(root.attrs, "viewBox")
        .map(num_list)
        .filter(|v| v.len() >= 4 && v[2] > 0.0 && v[3] > 0.0);
    let w = attr(root.attrs, "width").and_then(abs_len).filter(|v| *v > 0.0);
    let h = attr(root.attrs, "height").and_then(abs_len).filter(|v| *v > 0.0);
    let ratio = vb.as_ref().map(|v| v[2] / v[3]);
    let natural = match (w, h, ratio) {
        (Some(w), Some(h), _) => (w, h),
        (Some(w), None, Some(r)) => (w, w / r),
        (None, Some(h), Some(r)) => (h * r, h),
        (None, None, Some(r)) if r >= 2.0 => (300.0, 300.0 / r),
        (None, None, Some(r)) => (150.0 * r, 150.0),
        (w, h, None) => (w.unwrap_or(300.0), h.unwrap_or(150.0)),
    };
    let view = match &vb {
        Some(v) => [v[0], v[1], v[2], v[3]],
        None => [0.0, 0.0, natural.0, natural.1],
    };
    let self_closing = root.self_closing;
    Some(Viewport { root_attrs: root.attrs, self_closing, body_at, view, natural })
}
