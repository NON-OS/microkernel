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

//! The capsule's encoder and the kernel's decoder are one module pair:
//! every field survives the round trip, and every record the decoder
//! refuses the encoder will not produce.

use super::acpi::platform_decode::decode;
use super::acpi::platform_encode::encode;
use super::acpi::platform_wire::{Platform, GPIO, I2C, SD};

pub(super) fn platform(class: u8, hid: &[u8], windows: &[(u64, u64)], path: &[u8]) -> Platform {
    let mut h = [0u8; 8];
    h[..hid.len()].copy_from_slice(hid);
    let (windows, path) = (windows.to_vec(), path.to_vec());
    Platform { class, irq_flags: 0b1_1011, hid: h, uid: u64::MAX, gsiv: 0xFFFF_FFFF, windows, path }
}

#[test]
fn every_field_survives() {
    let five: Vec<(u64, u64)> = (0..5).map(|i| (0xFE00_0000 + i * 0x1000, 0x1000)).collect();
    let long = [b"\\".as_slice(), &[b'A'; 126]].concat();
    for p in [
        platform(I2C, b"AMDI0010", &[(0xFEDC_2000, 0x1000)], b"\\_SB.I2CA"),
        platform(SD, b"80860F14", &[(u64::MAX - 0xFFF, 0x1000)], b"\\X"),
        platform(GPIO, b"A", &five, &long),
    ] {
        let b = encode(&p).expect("encodes");
        assert_eq!(decode(&b), Some(p));
    }
}

#[test]
fn what_the_decoder_refuses_is_never_encoded() {
    let w = [(0xFE10_4000u64, 0x1000u64)];
    let six: Vec<(u64, u64)> = (0..6).map(|i| (i * 0x1000, 0x1000)).collect();
    let mut flagged = platform(I2C, b"INT3432", &w, b"\\_SB.I2C1");
    flagged.hid = *b"INT\x0032XX";
    for (p, why) in [
        (platform(9, b"INT3432", &w, b"\\_SB"), "class"),
        (platform(I2C, b"INT3432", &[], b"\\_SB"), "no window"),
        (platform(I2C, b"INT3432", &six, b"\\_SB"), "six windows"),
        (platform(I2C, b"", &w, b"\\_SB"), "no _HID"),
        (platform(I2C, b"INT-432", &w, b"\\_SB"), "a _HID of letters and digits"),
        (flagged, "NUL padding, then nothing"),
        (platform(I2C, b"INT3432", &w, b"_SB"), "no backslash"),
        (platform(I2C, b"INT3432", &w, b""), "no path"),
        (platform(I2C, b"INT3432", &w, b"\\_SB I2C"), "unprintable path"),
        (platform(I2C, b"INT3432", &w, &[b'\\'; 128]), "a path past 127"),
        (platform(I2C, b"INT3432", &w, &[b'\\'; 300]), "a path past a length byte"),
    ] {
        assert_eq!(encode(&p), None, "{why}");
    }
}
