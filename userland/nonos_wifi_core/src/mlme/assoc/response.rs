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

//! The AP's answer to the request: an association response, or a
//! reassociation response when the request was a reassociation.

use super::*;

impl Mlme {
    pub(in crate::mlme) fn on_assoc_response(&mut self, frame: &[u8]) -> MlmeOutput {
        if !from_bss_to_us(frame, &self.our_mac, &self.bssid) {
            return MlmeOutput::none();
        }
        let answer = match self.reassoc_from {
            Some(_) => parse_reassoc_response(frame),
            None => parse_assoc_response(frame),
        };
        match answer {
            Some((0, _aid)) => {
                self.start_handshake();
                MlmeOutput::none()
            }
            Some((STATUS_REJECTED_TEMPORARILY, _)) => MlmeOutput::none(),
            Some((status, _)) => self.fail(MlmeFailure::AssocRejected(status)),
            None => MlmeOutput::none(),
        }
    }
}
