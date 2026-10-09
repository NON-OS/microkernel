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

//! The part of a decode request that is the image's bytes alone: its size
//! from the header, the output buffer for it, and the decode into that. No
//! kernel call is made here, so the proofs drive the same path with any bytes.

use alloc::vec;
use alloc::vec::Vec;
use nonos_toolkit::image::{bmp, gif, jpeg, lz4_raw, png, types::{DecodeError, ImageSize}};

use crate::protocol::{DECODE_LZ4_PREFIX_LEN, E_BAD_LEN, E_INVAL, E_NOMEM, E_UNSUPPORTED, OP_DECODE_BMP, OP_DECODE_GIF, OP_DECODE_JPEG, OP_DECODE_LZ4_RAW, OP_DECODE_PNG};

pub const MAX_OUT_PIXELS: usize = 16_777_216;

fn peek_size(op: u16, img: &[u8]) -> Result<ImageSize, i32> {
    let r = match op {
        OP_DECODE_PNG => png::png_dimensions(img),
        OP_DECODE_BMP => bmp::bmp_dimensions(img),
        OP_DECODE_GIF => gif::gif_dimensions(img),
        OP_DECODE_JPEG => jpeg::parse_jpeg_header(img).map(|(s, _)| s),
        _ => return Err(E_INVAL),
    };
    r.map_err(map_decode_error)
}

fn decode_into(op: u16, img: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    match op {
        OP_DECODE_PNG => png::decoder::decode_png_argb8888(img, out),
        OP_DECODE_BMP => bmp::decode_bmp_argb8888(img, out),
        OP_DECODE_GIF => gif::decode_gif_argb8888(img, out),
        OP_DECODE_JPEG => jpeg::decode_jpeg_argb8888(img, out),
        OP_DECODE_LZ4_RAW => decode_lz4(img, out),
        _ => Err(DecodeError::Unsupported),
    }
}

pub fn decode_sized(op: u16, img: &[u8]) -> Result<(Vec<u32>, ImageSize), i32> {
    let size = if op == OP_DECODE_LZ4_RAW {
        if img.len() < DECODE_LZ4_PREFIX_LEN { return Err(E_BAD_LEN); }
        let w = u32::from_le_bytes([img[0], img[1], img[2], img[3]]);
        let h = u32::from_le_bytes([img[4], img[5], img[6], img[7]]);
        ImageSize::new(w, h).map_err(map_decode_error)?
    } else {
        peek_size(op, img)?
    };
    let count = size.pixel_count() as usize;
    if count == 0 || count > MAX_OUT_PIXELS {
        return Err(E_NOMEM);
    }
    let mut pixels = vec![0u32; count];
    let decoded = decode_into(op, img, &mut pixels).map_err(map_decode_error)?;
    Ok((pixels, decoded))
}

fn decode_lz4(body: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    if body.len() < DECODE_LZ4_PREFIX_LEN { return Err(DecodeError::Truncated); }
    let width = u32::from_le_bytes(body[0..4].try_into().map_err(|_| DecodeError::Truncated)?);
    let height = u32::from_le_bytes(body[4..8].try_into().map_err(|_| DecodeError::Truncated)?);
    lz4_raw::decode_lz4_raw_argb8888(width, height, &body[8..], out)
}

fn map_decode_error(err: DecodeError) -> i32 {
    match err { DecodeError::BadMagic => E_INVAL, DecodeError::Unsupported => E_UNSUPPORTED, DecodeError::BadDimensions => E_BAD_LEN, DecodeError::OutputTooSmall => E_BAD_LEN, DecodeError::Truncated => E_BAD_LEN }
}
