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

use crate::browser::image::store::Decoded;

use super::bitread::BitReader;
use super::decode_image::decode_image;
use super::header::read_header;
use super::read_transform::read_transform;
use super::transform::{apply_all, Transform};

const MAX_PIXELS: usize = 4_000_000;

/* Decode a lossless VP8L payload: the header, then the image stream. */
pub(super) fn decode_vp8l(data: &[u8]) -> Option<Decoded> {
    let mut br = BitReader::new(data);
    let (w, h) = read_header(&mut br)?;
    let px = image_stream(&mut br, w as usize, h as usize)?;
    Some(Decoded { w, h, px })
}

/// An image stream with no header, its size known from outside (an ALPH
/// chunk's lossless plane): up to four transforms, the entropy-coded main
/// image, then the transforms undone in reverse.
pub(super) fn decode_headless(data: &[u8], w: usize, h: usize) -> Option<Vec<u32>> {
    image_stream(&mut BitReader::new(data), w, h)
}

fn image_stream(br: &mut BitReader, w: usize, h: usize) -> Option<Vec<u32>> {
    if w.checked_mul(h)? > MAX_PIXELS {
        return None;
    }
    let mut xsize = w;
    let mut transforms: Vec<Transform> = Vec::new();
    while br.read_bit() == 1 {
        if transforms.len() >= 4 {
            return None;
        }
        transforms.push(read_transform(br, &mut xsize, h)?);
    }
    let px = decode_image(br, xsize, h, true)?;
    if br.eos {
        return None;
    }
    let (px, full_w) = apply_all(transforms, px, xsize, h);
    (full_w == w && px.len() == w * h).then_some(px)
}
