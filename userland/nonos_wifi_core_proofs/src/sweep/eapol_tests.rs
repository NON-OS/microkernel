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
//! EAPOL-Key frames (802.11-2020, Figure 12-32) by the standard's offsets:
//! a sound frame reads as built, trailing octets past its key data left
//! alone; a key data length past the frame, another packet type or a frame
//! shorter than the fixed fields is not one.

use super::rng::{each, within};
use nonos_wifi_core::eapol::parse::parse;

#[test]
fn key_frames_read_as_built_and_lies_are_refused() {
    each(|rng| {
        let kd = rng.span(0, 120);
        let mut f = rng.bytes(97);
        f[1] = 3;
        f.extend_from_slice(&(kd.len() as u16).to_be_bytes());
        f.extend_from_slice(&kd);
        f.extend(rng.span(0, 10));
        let k = parse(&f).expect("sound");
        assert_eq!(k.descriptor_type, f[4]);
        assert_eq!(k.key_info, u16::from_be_bytes([f[5], f[6]]));
        assert_eq!(k.key_length, u16::from_be_bytes([f[7], f[8]]));
        assert_eq!((&k.replay_counter[..], &k.nonce[..]), (&f[9..17], &f[17..49]));
        assert_eq!((&k.key_rsc[..], &k.mic[..]), (&f[65..73], &f[81..97]));
        assert_eq!(k.key_data, &kd[..]);
        assert!(within(&f, k.key_data));
        let room = f.len() - 99;
        if room < 0xffff {
            let mut lie = f.clone();
            let n = (room + 1 + rng.below(0xffff - room)) as u16;
            lie[97..99].copy_from_slice(&n.to_be_bytes());
            assert!(parse(&lie).is_none(), "key data length {n}");
        }
        let mut lie = f.clone();
        lie[1] = rng.pick(&[0u8, 1, 2, 4, 255]);
        assert!(parse(&lie).is_none(), "packet type {}", lie[1]);
        assert!(parse(&f[..rng.below(99)]).is_none(), "cut short");
    });
}

/// The message 3 the keyed sweeps lie with, honest: it connects.
#[test]
fn the_honest_message_3_connects() {
    use crate::ap_sim::*;
    use crate::handshake::sim::{configured, psk, ptk_for};
    use nonos_wifi_core::wpa::{akm::Akm, supplicant::State};
    let (pmk, kd) = (psk(), [&AP_RSNE_MIXED[..], &gtk_kde(1, &GTK)].concat());
    let mut sup = configured(Akm::Psk, pmk, &STA_RSNE_PSK, &AP_RSNE_MIXED);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = ptk_for(Akm::Psk, &pmk, &ANONCE);
    assert!(sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd)).reply.is_some());
    assert_eq!((sup.state(), sup.gtk()), (State::Connected, &GTK[..]));
}
