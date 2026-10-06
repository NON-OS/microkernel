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
use super::attr::{attr, style_prop};
use super::brush::{Brush, Shade};
use super::defs::{ref_id, Defs};
use super::fill::fill_polys;
use super::geom::device;
use super::path::parse_path;
use super::raster::Raster;
use super::shapes::shape_polys;
use super::transform::parse_transform;
use super::xml::next_tag;

/// The coverage over band `like` of the clipPath `value` names, for an
/// element with user map `t` and user box `bbox`: the union of its shapes
/// and paths. None when it names no clipPath, hiding the element.
pub(super) fn clip_mask(
    defs: &Defs,
    value: &str,
    t: &Affine,
    bbox: [f32; 4],
    like: &Raster,
) -> Option<Raster> {
    let (tag, mut pos) = next_tag(defs.doc, defs.element(ref_id(value)?)?)?;
    if tag.name != "clipPath" {
        return None;
    }
    let bbox_units =
        attr(tag.attrs, "clipPathUnits").is_some_and(|u| u.trim() == "objectBoundingBox");
    let unit = match bbox_units {
        true => Affine::translate(bbox[0], bbox[1])
            .then(&Affine::scale(bbox[2] - bbox[0], bbox[3] - bbox[1])),
        false => Affine::identity(),
    };
    let own = attr(tag.attrs, "transform").map_or(Affine::identity(), parse_transform);
    let base = t.then(&unit).then(&own);
    let mut mask = Raster::like(like)?;
    let brush = Brush { shade: Shade::Solid(0xffff_ffff), clip: None };
    while let Some((child, next)) = next_tag(defs.doc, pos).filter(|_| !tag.self_closing) {
        pos = next;
        if child.closing {
            if child.name == "clipPath" {
                break;
            }
            continue;
        }
        let polys = match (child.name, attr(child.attrs, "d")) {
            ("path", Some(d)) => parse_path(d),
            ("path", None) => continue,
            (name, _) => shape_polys(name, child.attrs),
        };
        let m = attr(child.attrs, "transform").map_or(base, |tr| base.then(&parse_transform(tr)));
        let style = attr(child.attrs, "style").unwrap_or("");
        let rule = style_prop(style, "clip-rule").or_else(|| attr(child.attrs, "clip-rule"));
        let evenodd = rule.is_some_and(|r| r.trim() == "evenodd");
        fill_polys(&mut mask, &device(&polys, &m), &brush, evenodd);
    }
    Some(mask)
}
