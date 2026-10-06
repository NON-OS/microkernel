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

//! Authentication responses from the chosen access point. Only a frame from
//! that BSS to this station counts (another station's refusal on the same
//! channel used to fail the join). Open System needs transaction 2 with
//! status 0; SAE frames go to the SAE exchange, which answers, accepts with a
//! PMK, or fails with its reason.

use super::failure::MlmeFailure;
use super::state::{Mlme, MlmeOutput};
use crate::dot11::auth::{auth_frame, parse_auth};
use crate::sae::frame::AUTH_ALG_SAE;
use crate::sae::SaeStep;
use crate::wpa::akm::Akm;

/// Open System: algorithm 0, the response is transaction 2.
const AUTH_ALG_OPEN: u16 = 0;
const OPEN_RESPONSE_SEQ: u16 = 2;

impl Mlme {
    pub(super) fn on_auth_response(&mut self, frame: &[u8]) -> MlmeOutput {
        let (us, bssid) = (self.our_mac, self.bssid);
        let Some(auth) = parse_auth(frame, &us, &bssid) else {
            return MlmeOutput::none();
        };
        if self.akm() != Some(Akm::Sae) {
            if auth.algorithm != AUTH_ALG_OPEN || auth.seq != OPEN_RESPONSE_SEQ {
                return MlmeOutput::none();
            }
            if auth.status != 0 {
                return self.fail(MlmeFailure::AuthRejected(auth.status));
            }
            return self.assoc_request();
        }
        if auth.algorithm != AUTH_ALG_SAE {
            return MlmeOutput::none();
        }
        let Some(sae) = self.sae.as_mut() else {
            return self.fail(MlmeFailure::NoEntropy);
        };
        match sae.on_frame(auth.seq, auth.status, auth.body) {
            SaeStep::Send { seq, status, body } => {
                let sc = self.next_seq();
                MlmeOutput::send(auth_frame(us, bssid, sc, AUTH_ALG_SAE, seq, status, &body))
            }
            SaeStep::Accepted => match sae.pmk() {
                Some((pmk, _pmkid)) => {
                    self.pmk = pmk;
                    self.assoc_request()
                }
                None => self.fail(MlmeFailure::NoEntropy),
            },
            SaeStep::Ignore => MlmeOutput::none(),
            SaeStep::Failed => {
                let why = sae.failure();
                match why {
                    Some(f) => self.fail(MlmeFailure::Sae(f)),
                    None => self.fail(MlmeFailure::NoEntropy),
                }
            }
        }
    }
}
