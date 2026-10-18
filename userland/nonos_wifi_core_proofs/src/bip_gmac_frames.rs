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

//! A broadcast Deauthentication and its BIP-GMAC-256 form, shared by the
//! BIP-GMAC-256 proofs.

use nonos_wifi_core::bip_gmac::{protect, BipGmacKey};

pub const K: [u8; 32] = [0x5a; 32];
pub const A2: [u8; 6] = [2, 0x11, 0x22, 0x33, 0x44, 0x55];

/// A broadcast Deauthentication: header, then reason code 7.
pub fn deauth() -> Vec<u8> {
    let mut f = vec![0xc0, 0x00, 0, 0];
    f.extend_from_slice(&[0xff; 6]);
    f.extend_from_slice(&A2);
    f.extend_from_slice(&A2);
    f.extend_from_slice(&[0x10, 0x00, 7, 0]);
    f
}

pub fn sealed(ipn: u64) -> Vec<u8> {
    let mut f = deauth();
    protect(&BipGmacKey::new(4, K).unwrap(), ipn, &mut f).unwrap();
    f
}
