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
//! commit for SAE. The beacon's RSNE and RSNXE are kept whole for the
//! downgrade check in message 3, and its rates for the association request.

use super::failure::MlmeFailure;
use super::state::{Mlme, MlmeOutput};
use crate::dot11::ies::{ap_rates, beacon_tags, find_element, EID_RSN, EID_RSNX};
use crate::dot11::parse::parse_beacon;
use crate::rsn::rsnxe::advertises_h2e;
use crate::rsn::{parse_rsne, select};
use crate::wpa::akm::Akm;
use crate::wpa::supplicant::ie::Ie;

impl Mlme {
    pub(super) fn on_beacon(&mut self, frame: &[u8]) -> MlmeOutput {
        let Some(info) = parse_beacon(frame) else {
            return MlmeOutput::none();
        };
        if info.ssid != self.ssid() || !self.channel_open(info.channel) {
            return MlmeOutput::none();
        }
        let Some(tags) = beacon_tags(frame) else {
            return MlmeOutput::none();
        };
        let Some(rsne_elem) = find_element(tags, EID_RSN) else {
            return self.fail(MlmeFailure::OpenNetwork);
        };
        let Some(rsne) = parse_rsne(&rsne_elem[2..]) else {
            return self.fail(MlmeFailure::MalformedRsne);
        };
        let rsnxe_elem = find_element(tags, EID_RSNX);
        let rates = ap_rates(tags);
        if rates.requires_ht {
            return self.fail(MlmeFailure::NeedsHt);
        }
        let h2e = rsnxe_elem.is_some_and(|e| advertises_h2e(&e[2..])) || rates.requires_h2e;
        let sel = match select(&rsne, h2e, self.policy) {
            Ok(sel) => sel,
            Err(e) => return self.fail(MlmeFailure::Select(e)),
        };
        if self.secret_too_long {
            return self.fail(MlmeFailure::BadPassphrase);
        }
        self.bssid = info.bssid;
        self.channel = info.channel;
        self.selection = Some(sel);
        self.ap_rsne = Ie::from_slice(rsne_elem);
        self.ap_rsnxe = Ie::from_option(rsnxe_elem);
        self.rates = Some(rates);
        if sel.akm == Akm::Sae {
            return self.start_sae_or_cached(sel.sae_h2e);
        }
        self.start_open()
    }
}
