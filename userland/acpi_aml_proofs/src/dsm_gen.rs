// NONOS Operating System (AGPL-3.0-or-later)
//! HID-over-I2C touchpads for the `_DSM` sweep (dsm_sweep_tests.rs): the
//! function-1 answer written as firmware writes it (a constant of each
//! width, Zero, One, or a named integer), the GUID inline, in a named
//! buffer, or one byte wrong, and the predicate either way round.

use alloc::vec::Vec;

use crate::dsm_aml::{integer, op, Rng, GUID};

/// A touchpad whose `_DSM` answers `v` for function 1, the GUID right when
/// `right`; its device body.
pub fn touchpad(r: &mut Rng, v: u64, right: bool) -> Vec<u8> {
    let mut guid = GUID;
    if !right {
        guid[r.below(16) as usize] ^= 1 << r.below(8);
    }
    let guid = op(&[0x11], &[&[0x0A, 0x10][..], &guid].concat());
    let named = r.below(3) == 0;
    let answer = if named { b"HIDA".to_vec() } else { integer(r, v) };
    let one = if r.below(2) == 0 { [0x93, 0x6A, 0x01] } else { [0x93, 0x01, 0x6A] };
    let zero = op(&[0xA0], &[0x93, 0x6A, 0x00, 0xA4, 0x11, 0x03, 0x01, 0x03]);
    let inner = [zero, op(&[0xA0], &[&one[..], &[0xA4], &answer].concat())].concat();
    let in_name = r.below(3) == 0;
    let arg0 = if in_name { b"GUIB".to_vec() } else { guid.clone() };
    let branch = op(&[0xA0], &[&[0x93, 0x68][..], &arg0, &inner].concat());
    let method = op(&[0x14], &[&b"_DSM"[..], &[0x0C], &branch].concat());
    let mut body = [
        &[0x08][..],
        b"_HID",
        &[0x0D],
        b"SYNA3602",
        &[0],
        &[0x08],
        b"_CID",
        &[0x0D],
        b"PNP0C50",
        &[0],
    ]
    .concat();
    if named {
        body.extend([&[0x08][..], b"HIDA", &integer(r, v)].concat());
    }
    if in_name {
        body.extend([&[0x08][..], b"GUIB", &guid].concat());
    }
    body.extend(method);
    body
}

/// `body` as a Device the enumeration walks.
pub fn device(body: &[u8]) -> Vec<u8> {
    op(&[0x5B, 0x82], &[&b"TPD0"[..], body].concat())
}
