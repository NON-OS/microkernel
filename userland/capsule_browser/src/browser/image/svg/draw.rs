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

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::affine::Affine;
use super::brush::{fade, Brush, Shade};
use super::clip::clip_mask;
use super::fill::fill_polys;
use super::geom::{bbox, device};
use super::grad_build::build;
use super::group::intersect;
use super::ink::Ink;
use super::state::Paint;
use super::stroke::stroke_polys;
use super::walk::Walk;

type P = [f32; 2];

impl Walk<'_, '_> {
    /// Paint user-space subpaths with `p`, fill then stroke as SVG orders
    /// them, inside the group clip and the element's own `clip-path`. An
    /// element whose clip reference does not resolve is not painted.
    pub(super) fn draw(&mut self, polys: &[Vec<P>], p: &Paint, own_clip: Option<&str>) {
        if polys.is_empty() {
            return;
        }
        let b = bbox(polys);
        let fill = p.fill.and_then(|ink| self.shade(ink, p.fill_op, b, &p.t));
        let stroke = p.stroke.and_then(|ink| self.shade(ink, p.stroke_op, b, &p.t));
        let group = p.clip.and_then(|i| self.masks.get(i));
        let own = match own_clip {
            Some(v) => match clip_mask(self.defs, v, &p.t, b, self.r) {
                Some(m) => Some(intersect(m, group)),
                None => return,
            },
            None => None,
        };
        let clip = own.as_ref().or(group);
        let dev = device(polys, &p.t);
        if let Some(shade) = fill {
            fill_polys(self.r, &dev, &Brush { shade, clip }, p.evenodd);
        }
        if let Some(shade) = stroke {
            stroke_polys(self.r, &dev, &Brush { shade, clip }, &p.pen.scaled(p.t.scale_avg()));
        }
    }

    fn shade(&self, ink: Ink, opacity: f32, b: [f32; 4], t: &Affine) -> Option<Shade> {
        match ink {
            Ink::Solid(c) => Some(Shade::Solid(fade(c, opacity))),
            Ink::Grad(i) => build(self.defs, i, b, t, opacity).map(|g| Shade::Grad(Box::new(g))),
        }
    }
}
