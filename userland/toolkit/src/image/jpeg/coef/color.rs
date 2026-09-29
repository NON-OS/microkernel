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

use crate::image::jpeg::ycbcr::ycbcr_to_argb8888;

/// How the frame's components map to colour.
#[derive(Clone, Copy, PartialEq)]
pub enum Model {
    Gray,
    YCbCr,
    Rgb,
    /// Four-component ink, stored inverted when an Adobe marker says so.
    Cmyk {
        inverted: bool,
    },
    /// Adobe YCCK: YCbCr-coded inverted C, M, Y plus K.
    Ycck,
}

/// The model for `n` components given the Adobe APP14 transform flag (if
/// any) and the component ids, as libjpeg guesses it.
pub fn model(n: usize, adobe: Option<u8>, ids: [u8; 4]) -> Model {
    match (n, adobe) {
        (1, _) => Model::Gray,
        (3, Some(0)) => Model::Rgb,
        (3, None) if ids[..3] == *b"RGB" => Model::Rgb,
        (3, _) => Model::YCbCr,
        (_, Some(2)) => Model::Ycck,
        (_, a) => Model::Cmyk { inverted: a.is_some() },
    }
}

/// One pixel from up to four component samples.
pub fn argb(m: Model, s: [u8; 4]) -> u32 {
    let rgb = |r: u32, g: u32, b: u32| 0xFF00_0000 | (r << 16) | (g << 8) | b;
    /* Ink over paper: each channel is what C, M or Y and K leave of 255. */
    let ink = |c: u8, k: u8| (255 - c as u32) * (255 - k as u32) / 255;
    match m {
        Model::Gray => rgb(s[0] as u32, s[0] as u32, s[0] as u32),
        Model::YCbCr => ycbcr_to_argb8888(s[0], s[1], s[2]),
        Model::Rgb => rgb(s[0] as u32, s[1] as u32, s[2] as u32),
        Model::Cmyk { inverted: false } => rgb(ink(s[0], s[3]), ink(s[1], s[3]), ink(s[2], s[3])),
        Model::Cmyk { inverted: true } => {
            rgb(ink(!s[0], !s[3]), ink(!s[1], !s[3]), ink(!s[2], !s[3]))
        }
        Model::Ycck => {
            let p = ycbcr_to_argb8888(s[0], s[1], s[2]);
            let ch = |sh: u32| (p >> sh) as u8;
            rgb(ink(ch(16), !s[3]), ink(ch(8), !s[3]), ink(ch(0), !s[3]))
        }
    }
}
