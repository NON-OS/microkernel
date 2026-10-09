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

use super::alpha::decode_alpha;
use super::filter::filter_mb;
use super::header::parse;
use super::macroblocks::macroblocks;
use super::planes::Plane;
use super::recon::Frame;
use crate::browser::image::shrink::Shrink;
use crate::browser::image::store::Decoded;

/* Pixels a lossy frame may have; its planes take 1.5 bytes each. */
const MAX_PIXELS: usize = 8_000_000;

/// Decode a VP8 key frame (RFC 6386) with an optional ALPH chunk, box
/// filtered to `size` (at most the natural size). Macroblocks are read
/// and reconstructed in order, the loop filter runs over the whole frame
/// once every prediction has read its unfiltered neighbours, and rows go
/// out through libwebp's chroma upsampling and colour transform.
pub(crate) fn decode_vp8(data: &[u8], alph: Option<&[u8]>, size: (u32, u32)) -> Option<Decoded> {
    let (hdr, mut br) = parse(data)?;
    let (w, h, mbw, mbh) = (hdr.w, hdr.h, hdr.mbw, hdr.mbh);
    if w * h > MAX_PIXELS {
        return None;
    }
    let alpha = match alph {
        Some(chunk) => Some(decode_alpha(chunk, w, h)?),
        None => None,
    };
    let (y, u, v) = (
        Plane::new(mbw * 16, mbh * 16)?,
        Plane::new(mbw * 8, mbh * 8)?,
        Plane::new(mbw * 8, mbh * 8)?,
    );
    let mut f = Frame { y, u, v, mbw };
    let strengths = macroblocks(&hdr, &mut br, &mut f)?;
    if hdr.filter > 0 {
        for (i, st) in strengths.iter().enumerate() {
            filter_mb(&mut f, (i % mbw, i / mbw), *st, hdr.filter == 1);
        }
    }
    let dst = (size.0.clamp(1, w as u32), size.1.clamp(1, h as u32));
    let mut shrink = Shrink::new((w as u32, h as u32), dst)?;
    super::yuv::emit(&f, (w, h), alpha.as_deref(), &mut |r, row| shrink.row(r, row));
    Some(shrink.finish())
}
