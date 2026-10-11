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

//! The connect reply: `[status i32][sent][recv][data][eapol][probe][deauth]
//! [to_us]` as little-endian u32s, then `[state u8][akm u8][ap code u16]`,
//! so the panel can show where a failed join stopped.

use crate::softmac::join::Counters;

#[derive(Default)]
pub struct ConnectResult {
    pub code: i32,
    pub counters: Counters,
    pub state: u8,
    pub akm: u8,
    pub ap_code: u16,
}

impl ConnectResult {
    pub fn code(code: i32) -> Self {
        Self { code, ..Default::default() }
    }

    /// Write the body after the 10-octet control header; its length.
    pub fn encode(&self, out: &mut [u8]) -> usize {
        let c = &self.counters;
        let words = [c.sent, c.recv, c.data, c.eapol, c.probe, c.deauth, c.to_us];
        out[0..4].copy_from_slice(&self.code.to_le_bytes());
        for (i, w) in words.iter().enumerate() {
            out[4 + 4 * i..8 + 4 * i].copy_from_slice(&w.to_le_bytes());
        }
        out[32] = self.state;
        out[33] = self.akm;
        out[34..36].copy_from_slice(&self.ap_code.to_le_bytes());
        36
    }
}
