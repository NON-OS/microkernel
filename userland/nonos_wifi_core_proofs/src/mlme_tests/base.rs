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

//! The station's addresses, its network, and how a join starts.

use nonos_wifi_core::mlme::{Entropy, JoinRequest, Mlme, SAE_ENTROPY};
use nonos_wifi_core::rsn::JoinPolicy;

pub(super) const STA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
pub(super) const AP: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
pub(super) const OTHER: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x99];
pub(super) const SSID: &[u8] = b"home";
pub(super) const PASS: &[u8] = b"correct horse";

pub(super) const SNONCE: [u8; 32] = [0x52; 32];

pub(super) fn entropy() -> Entropy {
    let mut sae = [0u8; SAE_ENTROPY];
    for (i, b) in sae.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(11);
    }
    Entropy { snonce: SNONCE, sae }
}

pub(super) fn join(policy: JoinPolicy) -> Mlme {
    Mlme::join(&JoinRequest { our_mac: STA, ssid: SSID, passphrase: PASS, policy, entropy: entropy() })
}
