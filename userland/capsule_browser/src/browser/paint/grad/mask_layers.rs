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

use alloc::vec::Vec;

use super::cache::{shape, CACHE};
use super::painter::Painter;
use super::split::split_top;

/// One gradient layer of a mask-image over its box, sampled a row at a
/// time; the alpha of each sample is the mask's coverage there.
pub(crate) struct MaskLayer(Painter);

/// The layers of mask value `v` over a w x h box. None when a layer is not
/// a gradient this module draws, and the box then paints unmasked.
pub(crate) fn mask_layers(v: &str, w: i32, h: i32) -> Option<Vec<MaskLayer>> {
    let mut guard = CACHE.lock();
    let cache = &mut *guard;
    let layers = split_top(v);
    let mut out = Vec::with_capacity(layers.len());
    for l in &layers {
        out.push(MaskLayer(Painter::new(shape(&mut cache.shapes, l)?, w, h)));
    }
    Some(out)
}

impl MaskLayer {
    /// Row y of the box from column x0, for out.len() samples.
    pub(crate) fn row(&self, y: i32, x0: i32, out: &mut [u32]) {
        self.0.row(y, x0, out);
    }
}
