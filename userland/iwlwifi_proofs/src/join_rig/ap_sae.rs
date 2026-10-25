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

//! The access point's SAE commit and confirm, answering the station's commit.

use nonos_wifi_core::sae::h2e::{derive_pt, pwe_from_pt};
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeStation, SaeStep};

use super::*;
use crate::gen3_model::Out;

impl Ap {
    /// The answer to the station's SAE commit `commit_in` with `status`, or
    /// `None` where the access point answers nothing at all.
    pub(super) fn sae_commit(&mut self, status: u16, commit_in: &[u8]) -> Option<Vec<Out>> {
        let Security::Sae { h2e } = self.sec else { return None };
        let pass: &[u8] = if self.sw.wrong_password { b"not the password" } else { PASS };
        let pwe = if h2e {
            pwe_from_pt(&derive_pt(SSID, pass, None).unwrap(), &AP, &STA).unwrap()
        } else {
            derive_pwe(pass, &vec![9u8; pass.len()], &AP, &STA).unwrap()
        };
        // As hostapd does: its commit, then its confirm at once.
        let mut ap = SaeStation::start(pwe, h2e, &[0x33; 32], &[0x44; 32]).unwrap();
        let SaeStep::Send { body: confirm, .. } = ap.on_frame(1, status, commit_in) else {
            return None;
        };
        let (_, s, commit) = ap.commit_frame();
        self.sae = Some(ap);
        let mut body = vec![3, 0, 1, 0];
        body.extend_from_slice(&s.to_le_bytes());
        body.extend_from_slice(&commit);
        let mut conf = vec![3, 0, 2, 0, 0, 0];
        conf.extend_from_slice(&confirm);
        Some(vec![rx(&mgmt(11, &body), 0), rx(&mgmt(11, &conf), 0)])
    }
}
