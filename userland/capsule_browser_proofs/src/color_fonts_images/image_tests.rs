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

//! Decoding through the engine's own path (natural size, plan, decode)
//! against PIL's decode of the same fixtures: lossy WebP with and without
//! alpha, lossless WebP exactly, and icons of both entry kinds.

use super::jpeg_tests::fixture;
use crate::browser::image::decode::{decode_body, natural};

/// Mean channel error of ARGB pixels against PIL's RGBA, alpha included,
/// colour counted only where PIL's pixel is not fully transparent.
fn argb_err(name: &str) -> f64 {
    let bytes = fixture(name);
    let nat = natural(&bytes).unwrap_or_else(|| panic!("{name}: no natural size"));
    let d = decode_body(&bytes, nat).unwrap_or_else(|e| panic!("{name}: {e}"));
    assert_eq!((d.w, d.h), nat, "{name} decodes at its natural size");
    let reference = fixture(&format!("{name}.rgba"));
    assert_eq!(d.px.len() * 4, reference.len(), "{name}: size differs from PIL's");
    let (mut sum, mut n) = (0u64, 0u64);
    for (p, r) in d.px.iter().zip(reference.chunks(4)) {
        sum += (p >> 24).abs_diff(r[3] as u32) as u64;
        n += 1;
        if r[3] != 0 {
            for (i, sh) in [16u32, 8, 0].iter().enumerate() {
                sum += ((p >> sh) & 0xff).abs_diff(r[i] as u32) as u64;
                n += 1;
            }
        }
    }
    sum as f64 / n as f64
}

#[test]
fn lossy_webp_with_and_without_alpha_matches_pil() {
    for name in ["lossy.webp", "lossy_alpha.webp"] {
        let e = argb_err(name);
        assert!(e <= 3.0, "{name}: mean error {e:.2} against PIL");
    }
}

#[test]
fn lossless_webp_is_exact() {
    assert_eq!(argb_err("lossless.webp"), 0.0);
}

#[test]
fn icons_decode_their_largest_entry() {
    for name in ["icon_bmp.ico", "icon_png.ico"] {
        let e = argb_err(name);
        assert!(e <= 0.5, "{name}: mean error {e:.2} against PIL");
    }
}
