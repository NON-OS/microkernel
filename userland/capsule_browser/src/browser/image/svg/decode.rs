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

use crate::browser::image::store::Decoded;

use alloc::vec::Vec;

use super::aspect::fit;
use super::attr::attr;
use super::defs::Defs;
use super::downsample::downsample;
use super::raster::{Raster, SS};
use super::state::Paint;
use super::viewport::viewport;
use super::walk::Walk;

/* Longest raster side: a vector is drawn at its display size up to this. */
pub(crate) const MAX_SIDE: u32 = 2048;
/* Samples in one band of the 2x canvas (4 MiB), and the tags one band may
 * visit, which bounds what nested use references can multiply. */
const BAND_PX: u32 = 1 << 20;
const MAX_TAGS: u32 = 200_000;

/// Rasterize the document at exactly `size` (each side 1..=MAX_SIDE): the
/// viewBox is mapped onto it and painted at 2x, a band of rows at a time
/// so the working canvas stays within BAND_PX samples, then box-filtered.
pub fn decode_svg(bytes: &[u8], size: (u32, u32)) -> Option<Decoded> {
    let doc = core::str::from_utf8(bytes).ok()?;
    let v = viewport(doc)?;
    let (ow, oh) = (size.0.clamp(1, MAX_SIDE), size.1.clamp(1, MAX_SIDE));
    let mut px = Vec::new();
    px.try_reserve_exact((ow * oh) as usize).ok()?;
    px.resize((ow * oh) as usize, 0);
    let par = attr(v.root_attrs, "preserveAspectRatio");
    let t = fit(par, v.view, (ow * SS) as f32, (oh * SS) as f32);
    let defs = Defs::scan(doc, v.view);
    let paint = Paint::root(t).derive(v.root_attrs, &defs);
    let rows = (BAND_PX / (ow * SS * SS)).clamp(1, oh);
    let mut y = 0;
    while y < oh && !v.self_closing {
        let n = rows.min(oh - y);
        let mut r = Raster::band(ow, n, y)?;
        let mut w = Walk { defs: &defs, r: &mut r, masks: Vec::new(), depth: 0, budget: MAX_TAGS };
        w.run(v.body_at, paint, false);
        downsample(&r, &mut px[(y * ow) as usize..((y + n) * ow) as usize], ow);
        y += n;
    }
    Some(Decoded { w: ow, h: oh, px })
}
