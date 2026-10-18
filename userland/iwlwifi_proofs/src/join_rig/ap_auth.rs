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

//! Open System and SAE authentication at the access point.

use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;
use nonos_wifi_core::sae::SaeStep;
use nonos_wifi_core::wpa::ptk::pmk;

use super::*;
use crate::gen3_model::Out;

impl Ap {
    pub(super) fn on_auth(&mut self, f: &[u8]) -> Vec<Out> {
        if self.sw.drop_first_auth && !self.dropped_auth {
            self.dropped_auth = true;
            return vec![];
        }
        let b = &f[MAC_HEADER_LEN..];
        let (alg, seq, status) =
            (u16::from_le_bytes([b[0], b[1]]), u16::from_le_bytes([b[2], b[3]]), u16::from_le_bytes([b[4], b[5]]));
        if let Some(code) = self.sw.refuse_auth {
            return vec![rx(&mgmt(11, &[b[0], b[1], 2, 0, code as u8, (code >> 8) as u8]), 0)];
        }
        let out = match (alg, seq) {
            (0, 1) => {
                self.pmk = pmk(if self.sw.wrong_pmk { b"another passphrase" } else { PASS }, SSID);
                vec![rx(&mgmt(11, &[0, 0, 2, 0, 0, 0]), 0)]
            }
            (3, 1) => match self.sae_commit(status, &b[6..]) {
                Some(out) => out,
                None => return vec![],
            },
            (3, 2) => {
                let Some(ap) = self.sae.as_mut() else { return vec![] };
                if matches!(ap.on_frame(2, status, &b[6..]), SaeStep::Accepted { .. }) {
                    self.pmk = ap.pmk().unwrap().0;
                }
                vec![]
            }
            _ => vec![],
        };
        if self.sw.silent_after_auth {
            self.sw.silent = true;
        }
        out
    }
}
