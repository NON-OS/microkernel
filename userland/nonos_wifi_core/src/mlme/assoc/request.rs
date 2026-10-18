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

//! The association (or reassociation) request: its elements.

use super::*;

impl Mlme {
    pub(in crate::mlme) fn assoc_request(&mut self) -> MlmeOutput {
        let Some(sel) = self.selection else {
            return self.fail(MlmeFailure::AssocRejected(0));
        };
        let rates = self.rates.unwrap_or_else(|| ap_rates(&[]));
        let rsne = self.own_rsne(&sel);
        // Each element is built into its own small buffer, then the request.
        let mut ssid_e = [0u8; 34];
        let mut sr_e = [0u8; 10];
        let mut esr_e = [0u8; 10];
        let (Some(ssid_n), Some(sr_n), Some(esr_n)) = (
            element(&mut ssid_e, 0, IE_SSID, self.ssid()),
            element(&mut sr_e, 0, EID_SUPPORTED_RATES, rates.supported()),
            element(&mut esr_e, 0, EID_EXT_SUPPORTED_RATES, rates.extended()),
        ) else {
            return self.fail(MlmeFailure::AssocRejected(0));
        };
        let esr: &[u8] = if rates.extended().is_empty() { &[] } else { &esr_e[..esr_n] };
        let rsnxe: &[u8] = if sel.sae_h2e { &STATION_RSNXE_H2E } else { &[] };
        // Extended Capabilities comes after the RSNE and before the RSNXE
        // (Table 9-35).
        let ext = self.ext_caps(&sel);
        let elements: [&[u8]; 6] = [&ssid_e[..ssid_n], &sr_e[..sr_n], esr, &rsne, ext, rsnxe];
        let seq = self.next_seq();
        let mut out = [0u8; 192];
        match self.request_frame(&mut out, seq, &elements) {
            Some(n) => {
                self.state = MlmeState::Associating;
                MlmeOutput::send(out[..n].to_vec())
            }
            None => self.fail(MlmeFailure::AssocRejected(0)),
        }
    }
}
