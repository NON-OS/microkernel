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

//! Drive the association forward. Management frames (beacon, authentication
//! response, association response) advance scan -> authenticate -> associate;
//! a deauthentication or disassociation from the chosen access point ends the
//! join with its reason code; EAPOL frames run the four-way handshake through
//! the supplicant.

use super::failure::MlmeFailure;
use super::state::{Mlme, MlmeOutput, MlmeState};
use crate::dot11::auth::parse_leave;
use crate::wpa::supplicant::State as SupState;

impl Mlme {
    /// Feed an 802.11 management frame received while joining.
    pub fn on_mgmt(&mut self, frame: &[u8]) -> MlmeOutput {
        if matches!(self.state, MlmeState::Authenticating | MlmeState::Associating | MlmeState::FourWay)
        {
            if let Some(reason) = parse_leave(frame, &self.our_mac, &self.bssid) {
                return self.fail(MlmeFailure::Left(reason));
            }
        }
        match self.state {
            MlmeState::Scanning => self.on_beacon(frame),
            MlmeState::Authenticating => self.on_auth_response(frame),
            MlmeState::Associating => self.on_assoc_response(frame),
            _ => MlmeOutput::none(),
        }
    }

    /// Feed the payload of an EAPOL-Key frame received while associated.
    pub fn on_eapol(&mut self, frame: &[u8]) -> MlmeOutput {
        if self.state != MlmeState::FourWay {
            return MlmeOutput::none();
        }
        let Some(sup) = self.supplicant.as_mut() else {
            return self.fail(MlmeFailure::AssocRejected(0));
        };
        let out = sup.step(frame);
        match sup.state() {
            SupState::Connected => {
                self.state = MlmeState::Connected;
                MlmeOutput { tx: out.reply }
            }
            SupState::Failed => match sup.failure() {
                Some(f) => self.fail(MlmeFailure::Handshake(f)),
                None => self.fail(MlmeFailure::AssocRejected(0)),
            },
            _ => MlmeOutput { tx: out.reply },
        }
    }
}
