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

//! Leaving the network on the person's word: a deauthentication to the AP,
//! protected under the pairwise key when the link has management frame
//! protection (an AP drops an unprotected one then), and the keys forgotten.

use super::state::Session;
use crate::dot11::auth::deauth_frame;
use crate::softmac::radio::Radio;

/// Reason 3: the station is leaving.
const REASON_LEAVING: u16 = 3;

impl Session {
    pub fn leave<R: Radio>(&mut self, r: &mut R) {
        if self.is_up() {
            let plain = deauth_frame(self.station.mac(), self.bssid, 0, REASON_LEAVING);
            let frame = if self.pmf { self.station.protect_mgmt(&plain) } else { Some(plain) };
            if let Some(f) = frame {
                r.send(&f);
            }
        }
        self.station.deassociate();
        self.queue.clear();
    }
}
