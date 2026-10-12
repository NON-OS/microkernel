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

//! Once associated the supplicant is configured, before message 1 can
//! arrive.

use super::*;

impl Mlme {
    // Associated: the AP sends message 1 next, which arrives via on_eapol.
    pub(super) fn start_handshake(&mut self) {
        let Some(sel) = self.selection else {
            self.fail(MlmeFailure::AssocRejected(0));
            return;
        };
        let own_rsne = station_rsne(&sel);
        let sup = Supplicant::configure(&Config {
            pmk: self.pmk,
            aa: self.bssid,
            spa: self.our_mac,
            snonce: self.snonce,
            akm: sel.akm,
            own_rsne: &own_rsne,
            own_rsnxe: sel.sae_h2e.then_some(&STATION_RSNXE_H2E[..]),
            ap_rsne: self.ap_rsne.as_slice(),
            ap_rsnxe: self.ap_rsnxe.as_option(),
            pmf: sel.pmf != Pmf::Off,
        });
        self.supplicant = Some(sup);
        self.state = MlmeState::FourWay;
    }
}
