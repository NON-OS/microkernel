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

use nonos_toolkit::image::{bmp, gif, png};

use crate::browser::image::shrink::fit;
use crate::browser::image::sniff::{Format, Probe};
use crate::browser::image::store::Decoded;

/* These decoders need the whole natural raster at once; past this many
 * pixels (16 MiB of ARGB) the transient buffer is refused. */
const MAX_PIXELS: u64 = 4_000_000;

/// A PNG, GIF or BMP decoded whole, then box-filtered to `size`.
pub(super) fn raster(bytes: &[u8], p: Probe, size: (u32, u32)) -> Result<Decoded, &'static str> {
    if p.w == 0 || p.h == 0 {
        return Err("bad image dimensions");
    }
    if p.w as u64 * p.h as u64 > MAX_PIXELS {
        return Err("image too large to decode whole");
    }
    let mut px = Vec::new();
    px.try_reserve_exact(p.w as usize * p.h as usize).map_err(|_| "no memory to decode")?;
    px.resize(p.w as usize * p.h as usize, 0u32);
    let got = match p.format {
        Format::Png => png::decoder::decode_png_argb8888(bytes, &mut px),
        Format::Bmp => bmp::decode_bmp_argb8888(bytes, &mut px),
        Format::Gif => gif::decode_gif_argb8888(bytes, &mut px),
        Format::Jpeg | Format::Webp => return Err("not a whole-raster format"),
    }
    .map_err(|_| "image decode failed")?;
    fit(Decoded { w: got.width, h: got.height, px }, size)
}

/// An icon's chosen entry: an embedded PNG or a DIB.
pub(super) fn ico(bytes: &[u8], size: (u32, u32)) -> Result<Decoded, &'static str> {
    let (w, h, data) =
        crate::browser::image::ico::pick(bytes).ok_or("icon directory unreadable")?;
    if data.starts_with(b"\x89PNG") {
        let p = Probe { format: Format::Png, w, h };
        let p = crate::browser::image::sniff::probe(data).unwrap_or(p);
        return raster(data, p, size);
    }
    let d = crate::browser::image::ico::decode_dib(data).ok_or("icon bitmap unreadable")?;
    fit(d, size)
}
