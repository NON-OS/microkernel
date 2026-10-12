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

//! Open System authentication for the PSK AKMs: the PMK from the passphrase,
//! then the authentication request (transaction 1).

use super::failure::MlmeFailure;
use super::state::{Mlme, MlmeOutput, MlmeState};
use crate::dot11::mgmt::auth_open;
use crate::wpa::psk::psk;

impl Mlme {
    pub(super) fn start_open(&mut self) -> MlmeOutput {
        let Some(key) = psk(self.secret(), self.ssid()) else {
            return self.fail(MlmeFailure::BadPassphrase);
        };
        self.pmk = key;
        let seq = self.next_seq();
        let mut out = [0u8; 64];
        match auth_open(&mut out, self.our_mac, self.bssid, seq) {
            Some(n) => {
                self.state = MlmeState::Authenticating;
                MlmeOutput::send(out[..n].to_vec())
            }
            None => self.fail(MlmeFailure::AuthRejected(0)),
        }
    }
}
