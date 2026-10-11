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

//! A WPA2-PSK access point as the station meets it on the air: it answers
//! Open System authentication and association, opens the four-way handshake,
//! checks message 2's MIC (a wrong passphrase gets a deauthentication with
//! reason 15, as hostapd sends), installs the pairwise key on message 4, and
//! from then on reads the station's protected data.

use crate::soft::frames::beacon;

pub struct SimAp {
    pub ssid: Vec<u8>,
    pub pass: Vec<u8>,
    pub channel: u8,
    pub hidden: bool,
    pub rsne: Vec<u8>,
    pub on: bool,
    pub tk: Option<[u8; 16]>,
    /// The plaintext bodies of the station's protected data frames.
    pub got: Vec<Vec<u8>>,
    pub(crate) ptk: Option<[u8; 48]>,
}

impl SimAp {
    pub fn new(ssid: &[u8], pass: &[u8], channel: u8, rsne: &[u8]) -> Self {
        let (ssid, pass, rsne) = (ssid.to_vec(), pass.to_vec(), rsne.to_vec());
        Self {
            ssid,
            pass,
            channel,
            hidden: false,
            rsne,
            on: true,
            tk: None,
            got: Vec::new(),
            ptk: None,
        }
    }

    pub fn beacon(&self) -> Vec<u8> {
        let name: &[u8] = if self.hidden { &[] } else { &self.ssid };
        beacon(8, [0xff; 6], name, self.channel, &self.rsne)
    }
}
