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

//! What the access point does with each frame the station sends, and its association.

use super::*;
use crate::ap_sim::message1;
use crate::gen3_model::Out;

impl Ap {
    /// Answer one frame the station sent.
    pub fn on_tx(&mut self, f: &[u8]) -> Vec<Out> {
        self.heard.push(f.to_vec());
        if self.sw.silent {
            return vec![];
        }
        match f[0] {
            0xB0 => self.on_auth(f),
            0x00 => self.on_assoc(),
            0xC0 => {
                self.deauths.push(f.to_vec());
                vec![]
            }
            0x08 => self.on_data(f),
            _ => vec![],
        }
    }

    pub(super) fn on_assoc(&mut self) -> Vec<Out> {
        let status = self.sw.refuse_assoc.unwrap_or(0);
        let mut body = vec![0x31, 0x04];
        body.extend_from_slice(&status.to_le_bytes());
        body.extend_from_slice(&0xC001u16.to_le_bytes());
        let mut out = vec![rx(&mgmt(1, &body), 0)];
        if status == 0 {
            let s = self.next_seq();
            out.push(rx(&eapol_from_ap(&message1(self.akm(), 1, &ANONCE), s), 0));
        }
        if self.sw.stray_m1 {
            let mut other = eapol_from_ap(&message1(self.akm(), 2, &[0xB2; 32]), 7);
            other[10] ^= 0x80;
            other[16] ^= 0x80;
            out.push(rx(&other, 0));
        }
        if self.sw.silent_after_m1 {
            self.sw.silent = true;
        }
        out
    }
}
