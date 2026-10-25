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
//! Every byte here is the attacker's: the beacon, its elements, the CCMP
//! ciphertext and AAD, the EAPOL-Key frame. No parser or check may panic or
//! read out of bounds, and a forged ciphertext or MIC is refused.

use nonos_wifi_core::ccmp::ccm::ccm_decrypt;
use nonos_wifi_core::dot11::parse::{find_ie, parse_beacon};
use nonos_wifi_core::eapol::mic::verify_mic;
use nonos_wifi_core::eapol::parse::parse;

/// Random buffers up to `max` long, from a fixed seed.
fn fuzz(mut s: u64, max: usize) -> impl FnMut() -> Vec<u8> {
    let mut next = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    move || {
        let len = next() as usize % (max + 1);
        (0..len).map(|_| next() as u8).collect()
    }
}

#[test]
fn the_beacon_and_element_parsers_take_any_bytes() {
    let mut buf = fuzz(0x1234_5678_9abc_def0, 80);
    for i in 0..200_000u32 {
        let b = buf();
        let _ = parse_beacon(&b);
        let _ = find_ie(&b, i as u8);
    }
}

#[test]
fn ccm_decrypt_refuses_any_forged_ciphertext() {
    let (key, nonce, mut out) = ([0x2bu8; 16], [0x11u8; 13], [0u8; 256]);
    let mut buf = fuzz(0xc0de_cafe_1234_abcd, 200);
    for _ in 0..100_000 {
        let (aad, input) = (buf(), buf());
        let aad = &aad[..aad.len().min(32)];
        assert_eq!(ccm_decrypt(&key, &nonce, aad, &input, &mut out), None);
    }
}

#[test]
fn the_eapol_parse_and_mic_take_any_frame() {
    let mut buf = fuzz(0xf00d_babe_1234_5678, 160);
    for i in 0..200_000u64 {
        let f = buf();
        let _ = parse(&f);
        let kck = [&i.to_le_bytes()[..], &[0; 8]].concat();
        assert!(!verify_mic(&kck, &f), "a forged MIC verified");
    }
}
