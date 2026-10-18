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

//! BIP-GMAC-256 refusals (IEEE Std 802.11-2020, 12.5.4.6): each bad input
//! is refused with its own reason, and a refused frame changes nothing.

use crate::bip_gmac_frames::{deauth, sealed, K};
use nonos_wifi_core::bip_gmac::{protect, verify, BipError, BipGmacKey, MMIE_LEN};

#[test]
fn a_changed_body_address_or_ipn_fails_the_mic() {
    let key = BipGmacKey::new(4, K).unwrap();
    for at in [24usize, 11, 4, 16] {
        let mut f = sealed(9);
        f[at] ^= 0x02;
        assert_eq!(verify(&key, &f, 0), Err(BipError::BadMic), "octet {at}");
    }
    let mut f = sealed(9);
    let n = f.len() - MMIE_LEN + 4;
    f[n] ^= 1;
    assert_eq!(verify(&key, &f, 0), Err(BipError::BadMic));
}

#[test]
fn replays_other_keys_and_unprotected_frames_are_refused() {
    let key = BipGmacKey::new(4, K).unwrap();
    assert_eq!(verify(&key, &sealed(9), 9), Err(BipError::Replay));
    let other = BipGmacKey::new(5, K).unwrap();
    assert_eq!(verify(&other, &sealed(9), 0), Err(BipError::WrongKey));
    assert_eq!(verify(&BipGmacKey::new(4, [1; 32]).unwrap(), &sealed(9), 0), Err(BipError::BadMic));
    assert_eq!(verify(&key, &deauth(), 0), Err(BipError::NoMmie));
}

#[test]
fn unicast_data_and_out_of_range_inputs_are_refused() {
    let key = BipGmacKey::new(4, K).unwrap();
    let mut uni = deauth();
    uni[4] = 0x02;
    assert_eq!(protect(&key, 1, &mut uni), Err(BipError::NotGroupAddressed));
    let mut data = deauth();
    data[0] = 0x08;
    assert_eq!(protect(&key, 1, &mut data), Err(BipError::NotManagement));
    for ipn in [0, 1 << 48] {
        assert_eq!(protect(&key, ipn, &mut deauth()), Err(BipError::BadIpn));
    }
    assert!(BipGmacKey::new(3, K).is_none() && BipGmacKey::new(8, K).is_none());
}
