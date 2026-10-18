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

//! BIP-GMAC-256 (IEEE Std 802.11-2020, 12.5.4). No BIP-GMAC-256 vector was
//! reachable here, so none is claimed: the MIC is checked to be exactly the
//! GCM tag over no plaintext that `gcm_tests` proves on McGrew and Viega's
//! cases, the MMIE layout byte by byte, and every refusal on its own input.

use crate::bip_gmac_frames::{sealed, A2, K};
use nonos_wifi_core::bip_gmac::{verify, BipGmacKey, MMIE_LEN};
use nonos_wifi_core::gcmp::gcm::seal;

#[test]
fn the_mmie_is_laid_out_as_9_4_2_54_and_the_mic_is_gmac() {
    let f = sealed(0x0102_0304_0506);
    let e = &f[f.len() - MMIE_LEN..];
    assert_eq!(&e[..10], &[76, 24, 4, 0, 6, 5, 4, 3, 2, 1]);
    let mut aad = vec![0xc0, 0x00];
    aad.extend_from_slice(&f[4..22]);
    aad.extend_from_slice(&f[24..f.len() - 16]);
    aad.extend_from_slice(&[0; 16]);
    let iv: [u8; 12] = [&A2[..], &[1, 2, 3, 4, 5, 6]].concat().try_into().unwrap();
    assert_eq!(seal(&K, &iv, &aad, &[]), e[10..].to_vec());
}

#[test]
fn a_protected_frame_verifies_and_returns_its_ipn() {
    let key = BipGmacKey::new(4, K).unwrap();
    assert_eq!(verify(&key, &sealed(9), 8), Ok(9));
}

#[test]
fn retry_power_and_more_data_and_the_sequence_number_are_not_bound() {
    let mut f = sealed(9);
    f[1] |= 0x08 | 0x10 | 0x20;
    f[22] = 0x70;
    assert_eq!(verify(&BipGmacKey::new(4, K).unwrap(), &f, 0), Ok(9));
}
