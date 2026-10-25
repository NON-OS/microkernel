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

//! The station's side of the MLME proofs: who joins what with which
//! passphrase, and the beacons an access point sends.

use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};
use nonos_wifi_core::mlme::{Entropy, JoinRequest, Mlme, SAE_ENTROPY};
use nonos_wifi_core::rsn::JoinPolicy;

pub const STA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
pub const AP: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
pub const OTHER: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x99];
pub const SSID: &[u8] = b"home";
pub const PASS: &[u8] = b"correct horse";
pub const SNONCE: [u8; 32] = [0x52; 32];

pub fn entropy() -> Entropy {
    let mut sae = [0u8; SAE_ENTROPY];
    for (i, b) in sae.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(11);
    }
    Entropy { snonce: SNONCE, sae, rekey: [0x6B; 32] }
}

pub fn join(policy: JoinPolicy) -> Mlme {
    Mlme::join(&JoinRequest {
        our_mac: STA,
        ssid: SSID,
        passphrase: PASS,
        policy,
        entropy: entropy(),
    })
}

// A beacon from `bssid` carrying the given elements after SSID and DS params.
pub fn beacon(bssid: [u8; 6], elements: &[&[u8]]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, 8), [0xFF; 6], bssid, bssid, 0).unwrap();
    f.extend_from_slice(&[0u8; 8]);
    f.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
    f.extend_from_slice(&[0, SSID.len() as u8]);
    f.extend_from_slice(SSID);
    f.extend_from_slice(&[3, 1, 6]);
    for e in elements {
        f.extend_from_slice(e);
    }
    f
}

pub fn rsne(akms: &[u8], caps: u16, group: u8) -> Vec<u8> {
    let mut b = vec![1, 0, 0x00, 0x0f, 0xac, group, 1, 0, 0x00, 0x0f, 0xac, 4];
    b.extend_from_slice(&(akms.len() as u16).to_le_bytes());
    for a in akms {
        b.extend_from_slice(&[0x00, 0x0f, 0xac, *a]);
    }
    b.extend_from_slice(&caps.to_le_bytes());
    let mut e = vec![48, b.len() as u8];
    e.extend_from_slice(&b);
    e
}
