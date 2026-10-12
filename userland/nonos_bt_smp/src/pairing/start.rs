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
use super::fresh::Fresh;
use super::identity::Identity;
use super::initiator::Initiator;
use super::io::Random;
use crate::error::SmpError;
use crate::pdu::{auth, keys, Pairing, PAIRING_REQUEST};
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
        Self::start_private(ours, None, a, b, rng, now_ms)
    }

    /// As `start`, with this host's identity to send when `ours` offers it
    /// (init_keys ID). Offering an identity without one to send, or any
    /// other key of ours, is refused: with Secure Connections the LTK is not
    /// sent, and this host signs no data (Vol 3, Part H, 3.6.1).
    pub fn start_private(
        ours: Pairing,
        local: Option<Identity>,
        a: [u8; 7],
        b: [u8; 7],
        rng: &mut dyn Random,
        now_ms: u64,
    ) -> Result<(Self, Vec<u8>), SmpError> {
        let offers_id = ours.init_keys & keys::ID != 0;
        if ours.auth_req & auth::SC == 0 || ours.max_key_size != 16 || ours.oob {
            return Err(SmpError::Refused);
        }
        if ours.init_keys & !keys::ID != 0 || offers_id != local.is_some() {
            return Err(SmpError::Refused);
        }
        let keys = draw::key_pair(rng)?;
        let request = pdu(PAIRING_REQUEST, &ours.params());
        let me = Fresh { ours, local, keys, a, b, now_ms }.into();
        Ok((me, request))
    }
}
