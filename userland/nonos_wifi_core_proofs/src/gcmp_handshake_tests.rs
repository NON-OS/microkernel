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

//! A GCMP-256 four-way handshake: the station's RSNE names the ciphers, the
//! supplicant derives the 64-octet PTK (message 3's MIC, under the longer
//! key's KCK, verifies only if it did), installs a 32-octet TK and takes a
//! 32-octet GTK for the GCMP-256 group cipher; a GTK of the wrong length
//! fails the handshake.

use crate::ap_sim::*;
use crate::cipher_sim::{GCMP_ONLY, GTK256};
use nonos_wifi_core::eapol::mic::verify_mic_kind;
use nonos_wifi_core::rsn::build::station_rsne;
use nonos_wifi_core::rsn::Cipher::Gcmp256;
use nonos_wifi_core::rsn::{Pmf, Selection};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::{Config, Failure, State, Supplicant};

const PMK: [u8; 32] = [0x6b; 32];

fn gcmp(akm: Akm) -> (Supplicant, [u8; 64]) {
    let sel = Selection { akm, pmf: Pmf::Off, sae_h2e: false, pairwise: Gcmp256, group: Gcmp256 };
    let own = station_rsne(&sel);
    let (aa, spa, snonce, own_rsne, ap_rsne) = (AA, SPA, SNONCE, &own[..], &GCMP_ONLY[..]);
    let (own_rsnxe, ap_rsnxe, pmf) = (None, None, false);
    let c = Config { pmk: PMK, aa, spa, snonce, akm, own_rsne, own_rsnxe, ap_rsne, ap_rsnxe, pmf };
    let mut ptk = [0u8; 64];
    assert!(akm.derive_ptk_into(&PMK, (&AA, &SPA), (&ANONCE, &SNONCE), &mut ptk));
    (Supplicant::configure(&c), ptk)
}

fn message3_with(akm: Akm, ptk: &[u8], gtk: &[u8]) -> Vec<u8> {
    let mut kd = GCMP_ONLY.to_vec();
    kd.extend_from_slice(&gtk_kde(1, gtk));
    message3(akm, 2, &ANONCE, ptk, &kd)
}

#[test]
fn a_gcmp_256_handshake_installs_32_octet_keys() {
    for akm in [Akm::Psk, Akm::Sae] {
        let (mut sup, ptk) = gcmp(akm);
        assert_eq!((sup.pairwise_cipher(), sup.group_cipher()), (Gcmp256, Gcmp256));
        let m2 = sup.step(&message1(akm, 1, &ANONCE)).reply.expect("message 2");
        assert!(verify_mic_kind(akm.mic_kind(), &ptk[..16], &m2), "{akm:?}: message 2 MIC");
        sup.step(&message3_with(akm, &ptk, &GTK256)).reply.expect("message 4");
        assert_eq!(sup.state(), State::Connected, "{akm:?}");
        assert_eq!(sup.tk(), &ptk[32..64], "{akm:?}: the 32-octet TK");
        assert_eq!(sup.gtk(), &GTK256, "{akm:?}: the 32-octet GTK");
    }
}

#[test]
fn a_ccmp_sized_gtk_under_a_gcmp_group_cipher_fails() {
    let (mut sup, ptk) = gcmp(Akm::Psk);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    sup.step(&message3_with(Akm::Psk, &ptk, &GTK));
    assert_eq!((sup.state(), sup.failure()), (State::Failed, Some(Failure::BadKeyData)));
}
