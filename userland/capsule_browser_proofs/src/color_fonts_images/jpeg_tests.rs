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

//! JPEG decoding against PIL's decode of the same files (fixtures/img,
//! written by PIL with its own RGBA decode beside each): baseline through
//! the streaming decoder, progressive and scaled through the coefficient
//! decoder. Mean error per channel must stay within 3 levels.

use nonos_toolkit::image::jpeg::coef::decode_scaled;
use nonos_toolkit::image::jpeg::decode_jpeg_argb8888;

pub fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/fixtures/img/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// Mean absolute RGB difference between ARGB pixels and PIL's RGBA bytes.
pub fn mean_err(got: &[u32], reference: &[u8]) -> f64 {
    assert_eq!(got.len() * 4, reference.len(), "size differs from PIL's");
    let mut sum = 0u64;
    for (p, r) in got.iter().zip(reference.chunks(4)) {
        for (i, sh) in [16u32, 8, 0].iter().enumerate() {
            sum += ((p >> sh) & 0xFF).abs_diff(r[i] as u32) as u64;
        }
    }
    sum as f64 / (got.len() * 3) as f64
}

pub(super) fn scaled(name: &str, shift: u32) -> (u32, u32, Vec<u32>) {
    let bytes = fixture(name);
    let mut px = Vec::new();
    let (w, h) = decode_scaled(&bytes, shift, 1 << 24, &mut |_, row| px.extend_from_slice(row))
        .unwrap_or_else(|e| panic!("{name}: {e:?}"));
    (w, h, px)
}

#[test]
fn progressive_and_baseline_match_pil() {
    for name in
        ["base_444.jpg", "gray_prog.jpg", "prog_444.jpg", "prog_422_rst.jpg", "prog_420.jpg"]
    {
        let (w, h, px) = scaled(name, 0);
        assert_eq!((w, h), (61, 45), "{name} size");
        let e = mean_err(&px, &fixture(&format!("{name}.rgba")));
        assert!(e <= 3.0, "{name}: mean error {e:.2} against PIL");
    }
}

#[test]
fn the_streaming_baseline_decoder_dequantizes_in_zigzag_order() {
    let mut px = vec![0u32; 61 * 45];
    decode_jpeg_argb8888(&fixture("base_444.jpg"), &mut px).expect("baseline decodes");
    let e = mean_err(&px, &fixture("base_444.jpg.rgba"));
    assert!(e <= 3.0, "mean error {e:.2}: DQT values are zigzag ordered");
    let mut px = vec![0u32; 61 * 45];
    decode_jpeg_argb8888(&fixture("prog_420.jpg"), &mut px).expect("progressive decodes");
    assert!(mean_err(&px, &fixture("prog_420.jpg.rgba")) <= 3.0);
}
