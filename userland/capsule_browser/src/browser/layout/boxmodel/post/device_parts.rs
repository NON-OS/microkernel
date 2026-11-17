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

use crate::browser::css::{BgLayer, BgLen, BgSize, ObjectFit, Shadow};
use crate::browser::layout::boxmodel::Rel;

use super::device_units::to_device;

/* The device-px forms of the CSS px a fragment's shadow, background
 * layer and object position carry. */
pub(super) fn shadow(mut s: Shadow, q: u32) -> Shadow {
    let d = |v: i16| to_device(v as i32, q).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    for l in s.layers.iter_mut() {
        (l.dx, l.dy, l.spread) = (d(l.dx), d(l.dy), d(l.spread));
        l.blur = d(l.blur.min(i16::MAX as u16) as i16).max(0) as u16;
    }
    s
}

fn rels(r: [Rel; 2], q: u32) -> [Rel; 2] {
    r.map(|(px, pm)| (to_device(px, q), pm))
}

pub(super) fn layer(mut l: BgLayer, q: u32) -> BgLayer {
    let len = |b: BgLen| match b {
        BgLen::Px(p) => BgLen::Px(to_device(p as i32, q).clamp(0, u16::MAX as i32) as u16),
        other => other,
    };
    if let BgSize::Wh(a, b) = l.size {
        l.size = BgSize::Wh(len(a), len(b));
    }
    l.pos = rels(l.pos, q);
    l
}

pub(super) fn object_fit(fit: ObjectFit, q: u32) -> ObjectFit {
    match fit {
        ObjectFit::Contain(p) => ObjectFit::Contain(rels(p, q)),
        ObjectFit::Cover(p) => ObjectFit::Cover(rels(p, q)),
        ObjectFit::Fill(p) => ObjectFit::Fill(rels(p, q)),
    }
}
