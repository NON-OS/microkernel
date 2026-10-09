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

use super::sniff::{probe, Format};
use super::store::Decoded;

use super::decode_full as full;

/// An image's natural size from its header, before any pixel is decoded:
/// SVG from its viewBox or width/height, an icon from its largest entry,
/// rasters from their own headers.
pub fn natural(bytes: &[u8]) -> Option<(u32, u32)> {
    if super::svg::is_svg(bytes) {
        return super::svg::natural_size(bytes);
    }
    if super::ico::is_ico(bytes) {
        return super::ico::pick(bytes).map(|(w, h, _)| (w, h));
    }
    probe(bytes).map(|p| (p.w, p.h)).filter(|&(w, h)| w > 0 && h > 0)
}

/// Decode to exactly `size` (a size from plan::target, never larger than
/// natural): vector art rasterizes there, a JPEG decodes near it through
/// its IDCT, everything else decodes whole and is box-filtered down.
pub(super) fn decode_to(bytes: &[u8], size: (u32, u32)) -> Result<Decoded, &'static str> {
    if super::svg::is_svg(bytes) {
        return super::svg::decode_svg(bytes, size).ok_or("svg parse failed");
    }
    if super::ico::is_ico(bytes) {
        return full::ico(bytes, size);
    }
    let p = probe(bytes).ok_or("unknown image format")?;
    match p.format {
        Format::Jpeg => super::jpeg::decode_jpeg(bytes, size),
        Format::Webp => super::webp::decode_webp(bytes, size).ok_or("webp decode failed"),
        _ => full::raster(bytes, p, size),
    }
}

/// The whole path for a body with no store: natural size, the size the
/// display hint asks for, and the decode. Host harnesses decode this way.
#[cfg(feature = "harness")]
pub fn decode_body(bytes: &[u8], hint: (u32, u32)) -> Result<Decoded, &'static str> {
    use super::plan::{target, MAX_RASTER_PX};
    let nat = natural(bytes).ok_or("unknown image format")?;
    decode_to(bytes, target(nat, hint, MAX_RASTER_PX))
}
