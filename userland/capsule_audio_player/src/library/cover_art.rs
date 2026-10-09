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

//! Reading the playing track's cover from its tags and decoding it, in this
//! process, with the toolkit's JPEG and PNG decoders, at about the size the
//! window shows it.

extern crate alloc;

use alloc::vec;

use nonos_app_skeleton::clients::vfs::VfsStream;
use nonos_toolkit::image::jpeg::coef::{cost, decode_scaled};
use nonos_toolkit::image::png::decoder::decode_png_argb8888;
use nonos_toolkit::image::png::png_dimensions;

use super::tags::{read_tags, CoverKind};
use crate::ui::art::picture::Picture;

/// The longest tag read for its cover. Album art is usually 50 to 500 KB.
const MAX_TAG: u32 = 2 * 1024 * 1024;
/// The side a cover is decoded to at least: the largest the window draws.
const TARGET: u32 = 320;
/// Coefficient storage one JPEG decode may hold.
const MAX_COEF: usize = 8 * 1024 * 1024;
/// The largest PNG decoded whole before it is shrunk.
const MAX_PNG_PIXELS: u64 = 2048 * 2048;

/// The cover of the file at `path`, if its tags carry one this can decode.
pub fn read(pid: u32, path: &str) -> Option<Picture> {
    let mut s = VfsStream::open(pid, path.as_bytes()).ok()?;
    let head = s.read_window(0, 10).ok()?;
    if head.len() < 10 || &head[..3] != b"ID3" {
        return None;
    }
    let size = head[6..10].iter().fold(0u32, |n, &b| (n << 7) | u32::from(b & 0x7F));
    let tag = s.read_window(0, size.saturating_add(10).min(MAX_TAG)).ok()?;
    let cover = read_tags(&tag, None).cover?;
    let bytes = tag.get(cover.at..cover.at.checked_add(cover.len)?)?;
    decode(cover.mime, bytes)
}

/// A cover's bytes as a picture, if they are a JPEG or PNG this decodes.
pub fn decode(kind: CoverKind, bytes: &[u8]) -> Option<Picture> {
    match kind {
        CoverKind::Jpeg => jpeg(bytes),
        CoverKind::Png => png(bytes),
        CoverKind::Other => None,
    }
}

fn jpeg(bytes: &[u8]) -> Option<Picture> {
    let c = cost(bytes).ok()?;
    // Read only as many coefficients as a 1/2, 1/4 or 1/8 decode needs
    // while that still covers the target.
    let mut shift = 0u32;
    while shift < 3 && (c.w >> (shift + 1)) >= TARGET && (c.h >> (shift + 1)) >= TARGET {
        shift += 1;
    }
    while shift < 3 && c.coef[shift as usize] > MAX_COEF {
        shift += 1;
    }
    let k = 8 >> shift;
    let w = (c.w * k).div_ceil(8) as usize;
    let h = (c.h * k).div_ceil(8) as usize;
    let mut px = vec![0u32; w.checked_mul(h)?];
    let (ow, oh) = decode_scaled(bytes, shift, MAX_COEF, &mut |y, row| {
        if let Some(out) = px.get_mut(y * w..y * w + row.len().min(w)) {
            out.copy_from_slice(&row[..out.len()]);
        }
    })
    .ok()?;
    Some(Picture { w: ow.min(w as u32), h: oh.min(h as u32), px })
}

fn png(bytes: &[u8]) -> Option<Picture> {
    let size = png_dimensions(bytes).ok()?;
    if u64::from(size.width) * u64::from(size.height) > MAX_PNG_PIXELS {
        return None;
    }
    let mut px = vec![0u32; (size.width * size.height) as usize];
    let got = decode_png_argb8888(bytes, &mut px).ok()?;
    Some(shrunk(Picture { w: got.width, h: got.height, px }))
}

/// `p` taken down to about twice the target, so a large PNG cover does not
/// hold megabytes for the window's life.
fn shrunk(p: Picture) -> Picture {
    let most = TARGET * 2;
    if p.w <= most && p.h <= most {
        return p;
    }
    let (w, h) = if p.w >= p.h {
        (most, (p.h * most / p.w).max(1))
    } else {
        ((p.w * most / p.h).max(1), most)
    };
    let mut px = vec![0u32; (w * h) as usize];
    for y in 0..h {
        let sy = (y * p.h / h) as usize;
        for x in 0..w {
            let sx = (x * p.w / w) as usize;
            px[(y * w + x) as usize] = p.px[sy * p.w as usize + sx];
        }
    }
    Picture { w, h, px }
}
