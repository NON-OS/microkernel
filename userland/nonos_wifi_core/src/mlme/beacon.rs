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

//! Choose the BSS and its security from the target network's beacon (or probe
//! response), and start authentication: Open System for the PSK AKMs, an SAE
//! commit for SAE. The frame is judged by the one rule the candidate list
//! also uses (`judge`); one heard on a channel the domain closes is treated
//! as not heard. The beacon's RSNE and RSNXE are kept whole for the
//! downgrade check in message 3, and its rates for the association request.

use super::failure::MlmeFailure;
use super::judge::judge;
use super::state::{Mlme, MlmeOutput};
use crate::dot11::parse::parse_beacon;
use crate::wpa::akm::Akm;
use crate::wpa::supplicant::ie::Ie;

impl Mlme {
    pub(super) fn on_beacon(&mut self, frame: &[u8]) -> MlmeOutput {
        let closed = parse_beacon(frame).is_some_and(|i| !self.channel_open(i.channel));
        if closed {
            return MlmeOutput::none();
        }
        let j = match judge(frame, self.ssid(), self.policy) {
            None => return MlmeOutput::none(),
            Some(Err(failure)) => return self.fail(failure),
            Some(Ok(j)) => j,
        };
        if self.secret_too_long {
            return self.fail(MlmeFailure::BadPassphrase);
        }
        let sel = j.selection;
        self.bssid = j.info.bssid;
        self.channel = j.info.channel;
        self.selection = Some(sel);
        self.ap_rsne = Ie::from_slice(j.rsne);
        self.ap_rsnxe = Ie::from_option(j.rsnxe);
        self.rates = Some(j.rates);
        if sel.akm == Akm::Sae {
            return self.start_sae_or_cached(sel.sae_h2e);
        }
        self.start_open()
    }
}
