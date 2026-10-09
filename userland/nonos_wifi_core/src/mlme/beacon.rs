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
use super::state::{Mlme, MlmeOutput, MlmeState};
use crate::dot11::auth::auth_frame;
use crate::dot11::ies::{ap_rates, beacon_tags, find_element, EID_RSN, EID_RSNX};
use crate::dot11::mgmt::auth_open;
use crate::dot11::parse::parse_beacon;
use crate::rsn::rsnxe::advertises_h2e;
use crate::rsn::{parse_rsne, select};
use crate::sae::frame::AUTH_ALG_SAE;
use crate::sae::h2e::{derive_pt, pwe_from_pt};
use crate::sae::hnp::derive_pwe;
use crate::sae::SaeStation;
use crate::wpa::akm::Akm;
use crate::wpa::psk::psk;
use crate::wpa::sha256::hmac_sha256_parts;
use crate::wpa::supplicant::ie::Ie;

impl Mlme {
    pub(super) fn on_beacon(&mut self, frame: &[u8]) -> MlmeOutput {
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

    // Derive the PWE and send the SAE commit.
    fn start_sae(&mut self, h2e: bool) -> MlmeOutput {
        let Some(entropy) = self.sae_entropy else {
            return self.fail(MlmeFailure::NoEntropy);
        };
        let (pw, ssid) = (self.secret(), self.ssid());
        let pwe = if h2e {
            derive_pt(ssid, pw, None).and_then(|pt| pwe_from_pt(&pt, &self.our_mac, &self.bssid))
        } else {
            derive_pwe(pw, &entropy[64..64 + pw.len()], &self.our_mac, &self.bssid)
        };
        let Some(pwe) = pwe else {
            return self.fail(MlmeFailure::BadPassphrase);
        };
        // rand and mask straight from the entropy; in the (2^-32) case that
        // either is not a usable scalar, draw replacements from it by HMAC.
        let mut sae = None;
        for attempt in 0u8..4 {
            let (rand, mask) = if attempt == 0 {
                let mut r = [0u8; 32];
                let mut m = [0u8; 32];
                r.copy_from_slice(&entropy[..32]);
                m.copy_from_slice(&entropy[32..64]);
                (r, m)
            } else {
                (
                    hmac_sha256_parts(&entropy[..64], &[b"rand", &[attempt]]),
                    hmac_sha256_parts(&entropy[..64], &[b"mask", &[attempt]]),
                )
            };
            sae = SaeStation::start(pwe, h2e, &rand, &mask);
            if sae.is_some() {
                break;
            }
        }
        let Some(sae) = sae else {
            return self.fail(MlmeFailure::NoEntropy);
        };
        let (seq, status, body) = sae.commit_frame();
        self.sae = Some(sae);
        self.state = MlmeState::Authenticating;
        let sc = self.next_seq();
        MlmeOutput::send(auth_frame(self.our_mac, self.bssid, sc, AUTH_ALG_SAE, seq, status, &body))
    }
}
