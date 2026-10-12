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

//! Starting pairing: the key pair is drawn and the Pairing Request built.

use super::draw;
use super::fields::pdu;
use super::initiator::Initiator;
use super::io::Random;
use super::state::State;
use super::step::Bond;
use crate::error::SmpError;
use crate::method::Method;
use crate::pdu::{auth, Pairing, PAIRING_REQUEST};
use alloc::vec::Vec;

impl Initiator {
    /// Pairing on a link where `a` is our address and `b` the peer's, in
    /// `addr56` form, as the link uses them. Returns the Pairing Request
    /// to send. A request without Secure Connections or a full-size key is
    /// refused: this host pairs no other way.
    pub fn start(
        ours: Pairing,
        a: [u8; 7],
        b: [u8; 7],
        rng: &mut dyn Random,
        now_ms: u64,
    ) -> Result<(Self, Vec<u8>), SmpError> {
        if ours.auth_req & auth::SC == 0 || ours.max_key_size != 16 || ours.oob {
            return Err(SmpError::Refused);
        }
        let keys = draw::key_pair(rng)?;
        let bond = Bond { ltk: [0; 16], irk: None, identity: None, authenticated: false };
        let request = pdu(PAIRING_REQUEST, &ours.params());
        let me = Self {
            state: State::AwaitResponse,
            ours,
            peer: None,
            keys,
            method: Method::JustWorks,
            peer_x: [0; 32],
            dh_key: [0; 32],
            passkey: 0,
            na: [0; 16],
            nb: [0; 16],
            cb: [0; 16],
            a,
            b,
            mac_key: [0; 16],
            bond,
            last_ms: now_ms,
        };
        Ok((me, request))
    }
}
