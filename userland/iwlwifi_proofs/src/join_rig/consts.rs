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

//! The station, the access point and the keys the rig fixes.

use nonos_wifi_core::mlme::{Entropy, JoinRequest, SAE_ENTROPY};
use nonos_wifi_core::rsn::JoinPolicy;

pub static SO_GF: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

pub const STA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
pub const AP: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
pub const SSID: &[u8] = b"home";
pub const PASS: &[u8] = b"correct horse";
pub const SNONCE: [u8; 32] = [0x52; 32];
pub const ANONCE: [u8; 32] = [0xA1; 32];
pub const TX_DEV: u64 = 0x6000_0000;

/// The WPA2-PSK RSNE an access point without WPA3 beacons.
pub const RSNE_PSK: [u8; 22] = [48, 20, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 4, 1, 0, 0, 0x0F, 0xAC, 2, 0, 0];

pub fn entropy() -> Entropy {
    let mut sae = [0u8; SAE_ENTROPY];
    for (i, b) in sae.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(11);
    }
    Entropy { snonce: SNONCE, sae }
}

pub fn request(policy: JoinPolicy) -> JoinRequest<'static> {
    JoinRequest { our_mac: STA, ssid: SSID, passphrase: PASS, policy, entropy: entropy() }
}
