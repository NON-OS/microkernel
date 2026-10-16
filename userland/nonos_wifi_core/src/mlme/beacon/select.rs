// NONOS Operating System (AGPL-3.0-or-later)

//! The target network's beacon chosen, and authentication started.

use super::super::failure::MlmeFailure;
use super::super::state::{Mlme, MlmeOutput, MlmeState};
use crate::dot11::ies::{ap_rates, beacon_tags, find_element, EID_RSN, EID_RSNX};
use crate::dot11::mgmt::auth_open;
use crate::dot11::parse::parse_beacon;
use crate::rsn::rsnxe::advertises_h2e;
use crate::rsn::{parse_rsne, select};
use crate::wpa::akm::Akm;
use crate::wpa::psk::psk;
use crate::wpa::supplicant::ie::Ie;

impl Mlme {
    pub(in crate::mlme) fn on_beacon(&mut self, frame: &[u8]) -> MlmeOutput {
        let Some(info) = parse_beacon(frame) else {
            return MlmeOutput::none();
        };
        if info.ssid != self.ssid() {
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
            return self.start_sae(sel.sae_h2e);
        }
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
