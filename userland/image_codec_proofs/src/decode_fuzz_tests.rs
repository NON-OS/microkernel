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

//! Damaged images through decode_sized, the codec's whole decode step. Seeded
//! from real PNG, BMP, GIF and JPEG files and a raw frame, then damaged where
//! headers keep sizes, lengths and markers, every input is either decoded
//! into a buffer that holds every pixel of the size it reports (the reply
//! path copies exactly that many into a surface) and no larger than the
//! codec's cap, or refused with one of the codec's errnos. The proofs run
//! with overflow checks on, so a wrapped sum anywhere on the way is a panic.

use crate::decode_sized::{decode_sized, MAX_OUT_PIXELS};
use crate::protocol::{
    E_BAD_LEN, E_INVAL, E_NOMEM, E_UNSUPPORTED, OP_DECODE_BMP, OP_DECODE_GIF, OP_DECODE_JPEG,
    OP_DECODE_LZ4_RAW, OP_DECODE_PNG,
};

const ROUNDS: usize = 200_000;

const PNG: [&[u8]; 4] = [
    include_bytes!("../../image_paint_proofs/fixtures/pngsuite/basn0g02.png"),
    include_bytes!("../../image_paint_proofs/fixtures/pngsuite/s01i3p01.png"),
    include_bytes!("../../image_paint_proofs/fixtures/pngsuite/tm3n3p02.png"),
    include_bytes!("../../image_paint_proofs/fixtures/misc/p_gray16.png"),
];
const BMP: [&[u8]; 3] = [
    include_bytes!("../../image_paint_proofs/fixtures/misc/bf16_565.bmp"),
    include_bytes!("../../image_paint_proofs/fixtures/misc/b_1bit.bmp"),
    include_bytes!("../../image_paint_proofs/fixtures/misc/alphabf32.bmp"),
];
const GIF: [&[u8]; 3] = [
    include_bytes!("../../image_paint_proofs/fixtures/misc/g_basic.gif"),
    include_bytes!("../../image_paint_proofs/fixtures/misc/g_transparent.gif"),
    include_bytes!("../../image_paint_proofs/fixtures/misc/gif_interlace_1x65535_43B.gif"),
];
const JPEG: [&[u8]; 3] = [
    include_bytes!("../../toolkit/image_tests/tests/fixtures/edge16.jpg"),
    include_bytes!("../../capsule_browser_proofs/fixtures/img/gray_prog.jpg"),
    include_bytes!("../../capsule_browser_proofs/fixtures/img/prog_420.jpg"),
];

fn raw() -> Vec<u8> {
    let mut f = 2u32.to_le_bytes().to_vec();
    f.extend_from_slice(&2u32.to_le_bytes());
    f.extend((0..16u8).map(|b| b.wrapping_mul(37)));
    f
}

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

/// A size-ish word: small, zero, near a power of two, past the cap, or any.
fn sizeish(s: &mut u64) -> u32 {
    match xorshift(s) % 6 {
        0 => 0,
        1 => (xorshift(s) % 64) as u32,
        2 => (1u32 << (xorshift(s) % 32)).wrapping_sub((xorshift(s) % 3) as u32),
        3 => u32::MAX - (xorshift(s) % 4) as u32,
        4 => 4096 + (xorshift(s) % 3) as u32,
        _ => xorshift(s) as u32,
    }
}

fn damage(s: &mut u64, img: &mut Vec<u8>) {
    for _ in 0..1 + xorshift(s) % 4 {
        if img.is_empty() {
            return;
        }
        let at = (xorshift(s) % img.len() as u64) as usize;
        match xorshift(s) % 8 {
            0 => img[at] ^= 1 << (xorshift(s) % 8),
            1 => img[at] = [0, 0xFF, 0x7F, 0x80][(xorshift(s) % 4) as usize],
            2 => img[at] = xorshift(s) as u8,
            3 | 4 => {
                let v = sizeish(s);
                let bytes = if xorshift(s) & 1 == 0 { v.to_be_bytes() } else { v.to_le_bytes() };
                let width = if xorshift(s) & 1 == 0 { 2 } else { 4 };
                let end = (at + width).min(img.len());
                img[at..end].copy_from_slice(&bytes[4 - width..][..end - at]);
            }
            5 => img.truncate(at),
            6 => img.insert(at, xorshift(s) as u8),
            _ => {
                let len = (xorshift(s) % 16) as usize;
                let from = (xorshift(s) % img.len() as u64) as usize;
                let chunk: Vec<u8> = img[from..(from + len).min(img.len())].to_vec();
                img.splice(at..at, chunk);
            }
        }
    }
}

/// What must hold for any op and any bytes. Returns whether it decoded.
fn check(op: u16, img: &[u8]) -> bool {
    match decode_sized(op, img) {
        Ok((pixels, size)) => {
            assert!(size.width > 0 && size.height > 0);
            assert!(size.pixel_count() <= pixels.len() as u64, "every reported pixel is held");
            assert!(pixels.len() <= MAX_OUT_PIXELS);
            true
        }
        Err(e) => {
            assert!([E_BAD_LEN, E_INVAL, E_NOMEM, E_UNSUPPORTED].contains(&e), "errno {e}");
            false
        }
    }
}

#[test]
fn the_seeds_decode() {
    for img in PNG {
        assert!(check(OP_DECODE_PNG, img));
    }
    for img in BMP {
        assert!(check(OP_DECODE_BMP, img));
    }
    for img in GIF {
        assert!(check(OP_DECODE_GIF, img));
    }
    for img in JPEG {
        assert!(check(OP_DECODE_JPEG, img));
    }
    assert!(check(OP_DECODE_LZ4_RAW, &raw()));
}

#[test]
fn damaged_images_decode_whole_or_are_refused() {
    let raw = raw();
    let mut s = 0x494D_4743_0000_0001u64;
    let mut decoded = [0usize; 5];
    for round in 0..ROUNDS {
        let kind = round % 5;
        let (op, seeds): (u16, &[&[u8]]) = match kind {
            0 => (OP_DECODE_PNG, &PNG),
            1 => (OP_DECODE_BMP, &BMP),
            2 => (OP_DECODE_GIF, &GIF),
            3 => (OP_DECODE_JPEG, &JPEG),
            _ => (OP_DECODE_LZ4_RAW, &[&raw[..]]),
        };
        let mut img = seeds[(xorshift(&mut s) % seeds.len() as u64) as usize].to_vec();
        damage(&mut s, &mut img);
        // Now and then the bytes of one format arrive under another's op.
        let op = if xorshift(&mut s) & 15 == 0 { [OP_DECODE_PNG, OP_DECODE_GIF][round & 1] } else { op };
        if check(op, &img) {
            decoded[kind] += 1;
        }
    }
    assert!(decoded.iter().all(|&n| n > 200), "every format reaches a whole decode: {decoded:?}");
}

#[test]
fn boundary_images() {
    for op in [OP_DECODE_PNG, OP_DECODE_BMP, OP_DECODE_GIF, OP_DECODE_JPEG, OP_DECODE_LZ4_RAW, 0, 99] {
        assert!(!check(op, &[]), "op {op} on nothing");
        assert!(!check(op, &[0xFF; 7]), "op {op} on seven bytes");
    }
    // A raw frame one pixel short, at the cap, and one past it.
    let mut short = raw();
    short.truncate(short.len() - 1);
    assert_eq!(decode_sized(OP_DECODE_LZ4_RAW, &short).err(), Some(E_BAD_LEN));
    let side = 4096u32;
    let mut at_cap = side.to_le_bytes().to_vec();
    at_cap.extend_from_slice(&side.to_le_bytes());
    assert_eq!(decode_sized(OP_DECODE_LZ4_RAW, &at_cap).err(), Some(E_BAD_LEN), "the cap, no bytes");
    let mut past = (side + 1).to_le_bytes().to_vec();
    past.extend_from_slice(&side.to_le_bytes());
    assert_eq!(decode_sized(OP_DECODE_LZ4_RAW, &past).err(), Some(E_NOMEM));
    let mut huge = u32::MAX.to_le_bytes().to_vec();
    huge.extend_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(decode_sized(OP_DECODE_LZ4_RAW, &huge).err(), Some(E_NOMEM));
    // Every seed cut short at every length is refused or decodes whole.
    for img in PNG.iter().chain(&BMP).chain(&GIF).chain(&JPEG) {
        for n in 0..img.len() {
            for op in [OP_DECODE_PNG, OP_DECODE_BMP, OP_DECODE_GIF, OP_DECODE_JPEG] {
                check(op, &img[..n]);
            }
        }
    }
}
