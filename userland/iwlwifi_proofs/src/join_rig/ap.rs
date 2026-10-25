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

//! The scripted access point: its state, and how it starts.

use nonos_wifi_core::sae::SaeStation;

use super::*;
use crate::ap_sim::AP_RSNE_MIXED;

/// The scripted access point.
pub struct Ap {
    pub sec: Security,
    pub sw: Switches,
    pub rsne: Vec<u8>,
    pub rsnxe: Option<Vec<u8>>,
    pub(super) sae: Option<SaeStation>,
    pub pmk: [u8; 32],
    pub ptk: [u8; 48],
    pub keyed: bool,
    pub seq: u16,
    pub pn: u64,
    /// The last protected frame it sent.
    pub last: Option<Vec<u8>>,
    /// Plaintext bodies (from LLC/SNAP) of protected frames it decrypted.
    pub got: Vec<Vec<u8>>,
    /// Every frame the station sent it.
    pub heard: Vec<Vec<u8>>,
    pub deauths: Vec<Vec<u8>>,
    pub eapol_replies: Vec<Vec<u8>>,
    pub(super) dropped_auth: bool,
}

impl Ap {
    pub fn new(sec: Security, sw: Switches) -> Self {
        let (rsne, rsnxe) = match sec {
            Security::Wpa2 => (RSNE_PSK.to_vec(), None),
            Security::Sae { h2e } => (AP_RSNE_MIXED.to_vec(), h2e.then(|| vec![244, 1, 0x20])),
        };
        Self {
            sec,
            sw,
            rsne,
            rsnxe,
            sae: None,
            pmk: [0; 32],
            ptk: [0; 48],
            keyed: false,
            seq: 0,
            pn: 0,
            last: None,
            got: Vec::new(),
            heard: Vec::new(),
            deauths: Vec::new(),
            eapol_replies: Vec::new(),
            dropped_auth: false,
        }
    }
}
