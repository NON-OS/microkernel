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

use crate::browser::css::calc::split_top::words;
use crate::browser::layout::boxmodel::{Clip, ClipR};

use super::clip_polygon::{inset, polygon};
use super::origin::parse_position;
use super::transform_fn::rel_len;

/// clip-path as the shape's bounding box. Some(None) is `none`; None is a
/// form this parser does not read (url() references, bare geometry boxes),
/// so the declaration drops.
pub(super) fn parse_clip(value: &str, fs: u32) -> Option<Option<Clip>> {
    let v = value.trim();
    if v.eq_ignore_ascii_case("none") {
        return Some(None);
    }
    let open = v.find('(')?;
    let inner = v.get(open + 1..v.len() - 1).filter(|_| v.ends_with(')'))?;
    let (shape, at) = match inner.find(" at ") {
        Some(i) => (&inner[..i], parse_position(&inner[i + 4..], fs)?),
        None => (inner, [(0, 500), (0, 500)]),
    };
    let radius = |w: &str| match w.to_ascii_lowercase().as_str() {
        "closest-side" => Some(ClipR::Closest),
        "farthest-side" => Some(ClipR::Farthest),
        _ => rel_len(w, fs).map(ClipR::Len),
    };
    let mut r = words(shape);
    Some(Some(match v[..open].trim().to_ascii_lowercase().as_str() {
        "inset" => inset(inner, fs)?,
        "circle" => {
            let rad = r.next().map_or(Some(ClipR::Closest), radius)?;
            Clip::Ellipse { cx: at[0], cy: at[1], rx: rad, ry: rad, circle: true }
        }
        "ellipse" => {
            let rx = r.next().map_or(Some(ClipR::Closest), radius)?;
            let ry = r.next().map_or(Some(ClipR::Closest), radius)?;
            Clip::Ellipse { cx: at[0], cy: at[1], rx, ry, circle: false }
        }
        "polygon" => polygon(inner, fs)?,
        _ => return None,
    }))
}
