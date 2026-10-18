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

//! A join from what a session kept instead of the passphrase: the PMK for
//! PSK and PSK-SHA256, the password token for SAE by hash-to-element. And
//! the password element, from the token or the password, after which the
//! passphrase is forgotten.

use super::state::{Mlme, SAE_ENTROPY};
use crate::rsn::parse_rsne;
use crate::sae::group::{point_from_xy, LEN};
use crate::sae::h2e::{derive_pt, pwe_from_pt};
use crate::sae::hnp::derive_pwe;
use crate::vault::wipe;
use nonos_p256::ProjectivePoint;

impl Mlme {
    /// Join PSK with `pmk` already derived; ignored unless 32 octets.
    pub fn given_pmk(&mut self, pmk: &[u8]) {
        if pmk.len() == 32 {
            self.pmk.copy_from_slice(pmk);
            self.pmk_given = true;
        }
    }

    /// Join SAE by hash-to-element with the token `pt` (x then y); ignored
    /// unless 64 octets.
    pub fn given_pt(&mut self, pt: &[u8]) {
        let mut t = [0u8; 2 * LEN];
        if pt.len() == t.len() {
            t.copy_from_slice(pt);
            self.pt = Some(t);
        }
    }

    /// The group management cipher suite the AP's beacon named, once a BSS
    /// was selected: a roam keeps it, as it keeps the rest of the security.
    pub fn ap_group_mgmt(&self) -> Option<u32> {
        parse_rsne(self.ap_rsne.as_slice().get(2..)?)?.group_mgmt
    }

    pub(super) fn forget_secret(&mut self) {
        wipe(&mut self.secret);
        self.secret_len = 0;
    }

    pub(super) fn pwe(&self, h2e: bool, entropy: &[u8; SAE_ENTROPY]) -> Option<ProjectivePoint> {
        let (pw, ssid, ours, bssid) = (self.secret(), self.ssid(), &self.our_mac, &self.bssid);
        if !h2e {
            return derive_pwe(pw, &entropy[64..64 + pw.len()], ours, bssid);
        }
        let pt = match &self.pt {
            Some(t) => point_from_xy(t[..LEN].try_into().ok()?, t[LEN..].try_into().ok()?)?,
            None => derive_pt(ssid, pw, None)?,
        };
        pwe_from_pt(&pt, ours, bssid)
    }
}
