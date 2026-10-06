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

//! BMP: palette (1, 4, 8 bit), 16-bit 5-5-5 and 5-6-5 bit fields, 24-bit,
//! 32-bit with and without an alpha mask, top-down rows, and the page's own
//! path from bytes to a ready raster.
use nonos_toolkit::image::bmp::{bmp_dimensions, decode_bmp_argb8888};
use nonos_toolkit::image::types::DecodeError;

use crate::browser::image::{ingest, Store};
use crate::fixtures::{expectations, fnv, read};

#[test]
fn bmp_fixtures_match_their_reference_decode() {
    let mut n = 0;
    for (name, want) in expectations().into_iter().filter(|e| e.0.ends_with(".bmp")) {
        let (w, h, hash) = want.expect("reference decodes");
        let mut out = std::vec![0u32; (w * h) as usize];
        let size =
            decode_bmp_argb8888(&read(&name), &mut out).unwrap_or_else(|e| panic!("{name}: {e:?}"));
        assert_eq!((size.width, size.height, fnv(&out)), (w, h, hash), "{name}");
        n += 1;
    }
    assert_eq!(n, 13);
}

#[test]
fn a_zero_alpha_channel_is_opaque_and_top_down_is_upright() {
    let mut out = std::vec![0u32; 13 * 7];
    decode_bmp_argb8888(&read("misc/rgb32_zero_alpha.bmp"), &mut out).unwrap();
    assert!(out.iter().all(|p| p >> 24 == 0xff));
    let top_down = read("misc/b_24_topdown.bmp");
    let size = bmp_dimensions(&top_down).unwrap();
    assert_eq!((size.width, size.height), (64, 48));
    let mut store = Store::new();
    ingest(&mut store, "data:td", &top_down);
    let d = store.ready("data:td").expect("the page decodes a top-down BMP");
    assert_eq!((d.w, d.h, d.px.len()), (64, 48, 64 * 48));
}

#[test]
fn malformed_bmps_are_refused() {
    let good = read("misc/b_8bit.bmp");
    let mut out = std::vec![0u32; 64 * 48];
    let mut rle = good.clone();
    rle[30] = 1;
    assert_eq!(decode_bmp_argb8888(&rle, &mut out), Err(DecodeError::Unsupported));
    let mut negative_w = good.clone();
    negative_w[18..22].copy_from_slice(&(-64i32).to_le_bytes());
    assert!(decode_bmp_argb8888(&negative_w, &mut out).is_err());
    assert_eq!(decode_bmp_argb8888(&good[..good.len() - 1], &mut out), Err(DecodeError::Truncated));
    assert_eq!(decode_bmp_argb8888(&good, &mut out[..10]), Err(DecodeError::OutputTooSmall));
}
