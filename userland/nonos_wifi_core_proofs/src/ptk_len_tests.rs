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

//! The PTK is as long as the pairwise cipher needs: 48 octets under
//! CCMP-128, 64 under GCMP-256. PRF-SHA1 output is a prefix stream, so the
//! longer PSK key extends the shorter; KDF-SHA256 takes the length as input,
//! so the SAE and PSK-SHA256 keys differ throughout. A temporal key carries
//! its cipher, and only that cipher's length is accepted.

use crate::ap_sim::{AA, ANONCE, SNONCE, SPA};
use nonos_wifi_core::rsn::Cipher::{Ccmp128, Gcmp256};
use nonos_wifi_core::rsn::TemporalKey;
use nonos_wifi_core::wpa::akm::Akm;

const PMK: [u8; 32] = [0x5a; 32];

fn ptk(akm: Akm, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    assert!(akm.derive_ptk_into(&PMK, (&AA, &SPA), (&ANONCE, &SNONCE), &mut out));
    out
}

#[test]
fn the_psk_key_for_gcmp_extends_the_ccmp_one() {
    let long = ptk(Akm::Psk, 64);
    assert_eq!(long[..48], Akm::Psk.derive_ptk(&PMK, &AA, &SPA, &ANONCE, &SNONCE));
    assert_ne!(long[48..], [0u8; 16], "the extra TK octets are derived");
}

#[test]
fn the_kdf_key_depends_on_its_length() {
    for akm in [Akm::Sae, Akm::PskSha256] {
        let (long, short) = (ptk(akm, 64), ptk(akm, 48));
        assert_eq!(short, akm.derive_ptk(&PMK, &AA, &SPA, &ANONCE, &SNONCE));
        assert_ne!(long[..16], short[..16], "{akm:?}: the KCK differs too");
        assert_eq!(long, ptk(akm, 64), "{akm:?}: deterministic");
    }
}

#[test]
fn a_temporal_key_has_its_ciphers_length() {
    assert_eq!((Ccmp128.tk_len(), Gcmp256.tk_len()), (16, 32));
    assert_eq!((Ccmp128.suite_type(), Gcmp256.suite_type()), (4, 9));
    assert!(TemporalKey::new(Ccmp128, &[1; 32]).is_none());
    assert!(TemporalKey::new(Gcmp256, &[1; 16]).is_none());
    let k = TemporalKey::new(Gcmp256, &[1; 32]).unwrap();
    assert_eq!((k.cipher(), k.bytes()), (Gcmp256, &[1u8; 32][..]));
}
