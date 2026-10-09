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
use super::color::CURRENT_COLOR;
use super::defs::Defs;
use super::gradient::fraction;
use super::ink::{inks, Ink};
use super::pen::Pen;
use super::transform::parse_transform;

/// Inherited paint state at one point of the element walk. `clip` is the
/// walk's current group clip mask, if any.
#[derive(Clone, Copy)]
pub(super) struct Paint {
    pub t: Affine,
    pub fill: Option<Ink>,
    pub stroke: Option<Ink>,
    pub pen: Pen,
    /// What currentColor paints: the inherited `color` property.
    pub color: u32,
    pub evenodd: bool,
    pub fill_op: f32,
    pub stroke_op: f32,
    pub clip: Option<usize>,
}

impl Paint {
    /// SVG paints black fill and no stroke by default.
    pub fn root(t: Affine) -> Self {
        let (fill, pen, color) = (Some(Ink::Solid(0xFF00_0000)), Pen::initial(), CURRENT_COLOR);
        let (fill_op, stroke_op, clip) = (1.0, 1.0, None);
        Paint { t, fill, stroke: None, pen, color, evenodd: false, fill_op, stroke_op, clip }
    }

    /// This element's state: its presentation attributes and inline style
    /// layered over the inherited values, its transform composed on. Group
    /// opacity is folded into both paints' opacity.
    pub fn derive(&self, attrs: &str, defs: &Defs) -> Paint {
        let mut p = *self;
        if let Some(tr) = attr(attrs, "transform") {
            p.t = p.t.then(&parse_transform(tr));
        }
        let style = attr(attrs, "style").unwrap_or("");
        let prop = |name: &str| style_prop(style, name).or_else(|| attr(attrs, name));
        inks(&mut p, prop, defs);
        p.pen.derive(prop);
        if let Some(v) = prop("fill-rule") {
            p.evenodd = v.trim().eq_ignore_ascii_case("evenodd");
        }
        let op = |name: &str| prop(name).and_then(fraction).map(|v| v.clamp(0.0, 1.0));
        p.fill_op = op("fill-opacity").unwrap_or(p.fill_op);
        p.stroke_op = op("stroke-opacity").unwrap_or(p.stroke_op);
        if let Some(o) = op("opacity") {
            (p.fill_op, p.stroke_op) = (p.fill_op * o, p.stroke_op * o);
        }
        p
    }
}
