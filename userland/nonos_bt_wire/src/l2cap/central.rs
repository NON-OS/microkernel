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

//! What a central answers on the LE signaling channel (CID 0x0005, Vol 3,
//! Part A, 2.1). This host opens no dynamic channel, so the only request
//! it acts on is a peripheral's Connection Parameter Update Request;
//! every other request is rejected as not understood (Vol 3, Part A, 4.1).

use super::conn_param::{ConnParams, UPDATE_ACCEPTED, UPDATE_REJECTED};
use super::signal::*;
use crate::error::WireError;
use alloc::vec::Vec;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Answer {
    /// The signaling frame to send back, if any.
    pub reply: Option<Vec<u8>>,
    /// Timing accepted for the link; the caller applies it with LE
    /// Connection Update (Vol 4, Part E, 7.8.18).
    pub update: Option<ConnParams>,
}

/// The answer to one frame from the peer. A frame that cannot be parsed
/// has no identifier to answer with, so it is returned as an error and
/// nothing is sent.
pub fn answer_central(frame: &[u8]) -> Result<Answer, WireError> {
    let s = parse_signal(frame)?;
    match s.code {
        // Rejecting a reject or a response could loop between two hosts.
        COMMAND_REJECT | CONN_PARAM_UPDATE_RESPONSE | DISCONNECTION_RESPONSE => {
            Ok(Answer::default())
        }
        CONN_PARAM_UPDATE_REQUEST => match ConnParams::parse(s.data) {
            Ok(p) => {
                let ok = p.acceptable();
                let result = if ok { UPDATE_ACCEPTED } else { UPDATE_REJECTED };
                let reply = signal(CONN_PARAM_UPDATE_RESPONSE, s.id, &result.to_le_bytes())?;
                Ok(Answer { reply: Some(reply), update: ok.then_some(p) })
            }
            Err(_) => {
                Ok(Answer { reply: Some(command_reject(s.id, NOT_UNDERSTOOD)?), update: None })
            }
        },
        _ => Ok(Answer { reply: Some(command_reject(s.id, NOT_UNDERSTOOD)?), update: None }),
    }
}
