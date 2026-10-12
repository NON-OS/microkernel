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

//! The two BIP-GMAC-256 readings (IEEE Std 802.11-2020, 12.5.4) that no
//! printed vector here confirms, each pinned so a correction shows up as one
//! changed line in the core and one failing proof here.

use crate::bip_gmac_frames::{sealed, K};
use nonos_wifi_core::bip_gmac::{ipn_octets, verify, BipError, BipGmacKey, FC1_MASK};

#[test]
fn reading_1_retry_power_management_and_more_data_are_masked() {
    assert_eq!(FC1_MASK, 0xc7);
    let key = BipGmacKey::new(4, K).unwrap();
    for bit in [0x08u8, 0x10, 0x20] {
        let mut f = sealed(9);
        f[1] ^= bit;
        assert_eq!(verify(&key, &f, 0), Ok(9), "bit {bit:#04x}");
    }
    for bit in [0x01u8, 0x02, 0x04, 0x40] {
        let mut f = sealed(9);
        f[1] ^= bit;
        assert_eq!(verify(&key, &f, 0), Err(BipError::BadMic), "bit {bit:#04x}");
    }
}

#[test]
fn reading_2_the_ipn_enters_the_nonce_most_significant_octet_first() {
    assert_eq!(ipn_octets(0x0102_0304_0506), [1, 2, 3, 4, 5, 6]);
    assert_eq!(ipn_octets(1), [0, 0, 0, 0, 0, 1]);
}
