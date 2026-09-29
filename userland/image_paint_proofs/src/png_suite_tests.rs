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

//! PNG conformance: every PngSuite image (all color types and bit depths,
//! Adam7, tRNS, odd sizes, chunk ordering) must decode to the reference
//! raster, and every corrupt one must be refused without a panic.
use nonos_toolkit::image::png::decoder::decode_png_argb8888;

use crate::fixtures::{expectations, fnv, read};

#[test]
fn every_png_fixture_matches_its_reference_decode() {
    let (mut exact, mut refused) = (0, 0);
    for (name, want) in expectations().into_iter().filter(|e| e.0.ends_with(".png")) {
        let bytes = read(&name);
        let mut out = std::vec![0u32; 4_000_000];
        let got = decode_png_argb8888(&bytes, &mut out);
        match want {
            Some((w, h, hash)) => {
                let size = got.unwrap_or_else(|e| panic!("{name}: {e:?}"));
                assert_eq!((size.width, size.height), (w, h), "{name}");
                assert_eq!(fnv(&out[..(w * h) as usize]), hash, "{name} pixels");
                exact += 1;
            }
            None => {
                assert!(got.is_err(), "{name} is corrupt and must be refused");
                refused += 1;
            }
        }
    }
    /* 161 valid PngSuite images plus the two misc PNGs; 14 corrupt x* files. */
    assert_eq!((exact, refused), (163, 14));
}

#[test]
fn trns_keys_grey_and_rgb_at_the_image_bit_depth() {
    for (name, transparent) in [("pngsuite/tbrn2c08.png", true), ("pngsuite/tbbn0g04.png", true)] {
        let mut out = std::vec![0u32; 32 * 32];
        decode_png_argb8888(&read(name), &mut out).unwrap();
        assert_eq!(out.iter().any(|p| p >> 24 == 0), transparent, "{name}");
        assert!(out.iter().any(|p| p >> 24 == 255), "{name} keeps opaque pixels");
    }
}
