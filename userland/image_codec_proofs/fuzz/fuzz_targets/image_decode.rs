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

//! image_codec's decode step, compiled from the capsule by
//! image_codec_proofs: decode_sized under PNG, BMP, GIF, JPEG and raw LZ4.
//! None panics, and overflow checks are on, so a wrapped sum on the way is
//! one. Every input is either decoded into a buffer that holds every pixel
//! of the size it reports and no more than the codec's cap, or refused with
//! one of the codec's errnos.

#![no_main]

use image_codec_proofs::decode_sized::{decode_sized, MAX_OUT_PIXELS};
use image_codec_proofs::protocol::{
    E_BAD_LEN, E_INVAL, E_NOMEM, E_UNSUPPORTED, OP_DECODE_BMP, OP_DECODE_GIF, OP_DECODE_JPEG,
    OP_DECODE_LZ4_RAW, OP_DECODE_PNG,
};
use libfuzzer_sys::fuzz_target;

/// The op a leading byte picks; seed.py writes the same order.
const OPS: [u16; 5] = [OP_DECODE_PNG, OP_DECODE_BMP, OP_DECODE_GIF, OP_DECODE_JPEG, OP_DECODE_LZ4_RAW];

fuzz_target!(|data: &[u8]| {
    let Some((&pick, img)) = data.split_first() else {
        return;
    };
    let op = OPS[usize::from(pick) % OPS.len()];
    match decode_sized(op, img) {
        Ok((pixels, size)) => {
            assert!(size.width > 0 && size.height > 0, "an empty image decoded");
            assert!(size.pixel_count() <= pixels.len() as u64, "a reported pixel not held");
            assert!(pixels.len() <= MAX_OUT_PIXELS, "a buffer past the cap");
        }
        Err(e) => assert!([E_BAD_LEN, E_INVAL, E_NOMEM, E_UNSUPPORTED].contains(&e), "errno {e}"),
    }
});
