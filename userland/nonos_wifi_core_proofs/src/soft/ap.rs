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

//! A WPA2-PSK access point, CCMP-128 or GCMP-256 as its RSNE offers and the
//! station's message 2 chooses, as the station meets it on the air: it answers
//! Open System authentication and association, opens the four-way handshake,
//! checks message 2's MIC (a wrong passphrase gets a deauthentication with
//! reason 15, the four-way handshake timeout an AP reports for a MIC that
//! fails), installs the pairwise key on message 4, and from then on reads the
//! station's protected data.

use crate::ap_sim::AA;
use crate::soft::frames::beacon_from;
use nonos_wifi_core::rsn::{Cipher, TemporalKey};

pub struct SimAp {
    pub bssid: [u8; 6],
    pub ssid: Vec<u8>,
    pub pass: Vec<u8>,
    pub channel: u8,
    pub hidden: bool,
    pub rsne: Vec<u8>,
    pub on: bool,
    /// Refuse every (re)association, status 17 (AP unable to handle more).
    pub refuse: bool,
    pub tk: Option<TemporalKey>,
    /// The plaintext bodies of the station's protected data frames.
    pub got: Vec<Vec<u8>>,
    /// The PTK and the pairwise cipher the station's message 2 named.
    pub(crate) ptk: Option<(Cipher, Vec<u8>)>,
}

impl SimAp {
    pub fn new(ssid: &[u8], pass: &[u8], channel: u8, rsne: &[u8]) -> Self {
        let (ssid, pass, rsne) = (ssid.to_vec(), pass.to_vec(), rsne.to_vec());
        Self {
            bssid: AA,
            ssid,
            pass,
            channel,
            hidden: false,
            rsne,
            on: true,
            refuse: false,
            tk: None,
            got: Vec::new(),
            ptk: None,
        }
    }

    pub fn beacon(&self) -> Vec<u8> {
        let name: &[u8] = if self.hidden { &[] } else { &self.ssid };
        beacon_from(self.bssid, 8, [0xff; 6], (name, self.channel, &self.rsne))
    }
}
