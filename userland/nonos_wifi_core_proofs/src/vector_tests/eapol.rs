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
//! The EAPOL-Key frame parsed or refused, its MIC checked by round trip,
//! tamper and wrong key, and message 2 built under a KCK from the PMK.

use nonos_wifi_core::eapol::build::build_key_frame;
use nonos_wifi_core::eapol::mic::{compute_mic, verify_mic};
use nonos_wifi_core::eapol::parse::{
    parse, EAPOL_TYPE_KEY, HEADER_LEN, KEY_INFO_MIC, KEY_INFO_PAIRWISE, MIC_LEN, MIC_OFFSET,
};
use nonos_wifi_core::wpa::ptk::{pmk, ptk};

fn frame(key_info: u16, nonce: &[u8; 32], key_data: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; HEADER_LEN + key_data.len()];
    f[..2].copy_from_slice(&[2, EAPOL_TYPE_KEY]);
    f[2..4].copy_from_slice(&((HEADER_LEN - 4 + key_data.len()) as u16).to_be_bytes());
    f[4] = 2;
    f[5..7].copy_from_slice(&key_info.to_be_bytes());
    f[17..49].copy_from_slice(nonce);
    f[97..99].copy_from_slice(&(key_data.len() as u16).to_be_bytes());
    f[HEADER_LEN..].copy_from_slice(key_data);
    f
}

#[test]
fn an_eapol_key_frame_parses_or_is_refused() {
    let f = frame(KEY_INFO_MIC | KEY_INFO_PAIRWISE, &[0x5a; 32], &[0xaa, 0xbb]);
    let k = parse(&f).unwrap();
    assert_eq!(k.key_info, KEY_INFO_MIC | KEY_INFO_PAIRWISE);
    assert_eq!((k.nonce, k.key_data), ([0x5a; 32], &[0xaa, 0xbb][..]));
    assert!(parse(&f[..HEADER_LEN - 1]).is_none(), "too short");
    let mut g = f.clone();
    g[1] = 0;
    assert!(parse(&g).is_none(), "not a key frame");
    let mut g = f.clone();
    g[97..99].copy_from_slice(&100u16.to_be_bytes());
    assert!(parse(&g).is_none(), "the key data length overruns");
}

#[test]
fn the_mic_verifies_and_refuses_a_tamper_or_the_wrong_kck() {
    let kck = [0x11u8; 16];
    let mut f = frame(KEY_INFO_MIC, &[0x22; 32], &[1, 2, 3, 4]);
    let mic = compute_mic(&kck, &f);
    f[MIC_OFFSET..MIC_OFFSET + MIC_LEN].copy_from_slice(&mic);
    assert!(verify_mic(&kck, &f) && !verify_mic(&[0u8; 16], &f), "only under its KCK");
    f[20] ^= 1;
    assert!(!verify_mic(&kck, &f), "a tampered frame");
}

#[test]
fn message_2_built_under_the_derived_kck_verifies_and_parses() {
    let (anonce, snonce) = ([0xaa; 32], [0xbb; 32]);
    let p =
        ptk(&pmk(b"password", b"IEEE"), &[0, 0, 0, 0, 0, 1], &[0, 0, 0, 0, 0, 2], &anonce, &snonce);
    let (kck, replay, mut out) = (&p[..16], [0, 0, 0, 0, 0, 0, 0, 1], [0u8; 128]);
    let info = KEY_INFO_MIC | KEY_INFO_PAIRWISE;
    let n = build_key_frame(&mut out, info, &replay, &snonce, &[0xde, 0xad], kck).unwrap();
    assert!(verify_mic(kck, &out[..n]) && !verify_mic(&[0u8; 16], &out[..n]));
    let k = parse(&out[..n]).unwrap();
    assert_eq!((k.nonce, k.replay_counter, k.key_data), (snonce, replay, &[0xde, 0xad][..]));
}
