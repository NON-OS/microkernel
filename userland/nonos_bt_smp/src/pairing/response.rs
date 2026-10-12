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

//! The Pairing Response (Vol 3, Part H, 3.5.2): both sides' features fix
//! the method, then our public key goes out (Vol 3, Part H, 2.3.5.6.1).

use super::fields::pdu;
use super::initiator::Initiator;
use super::state::State;
use super::step::Step;
use crate::method::{select, Method};
use crate::pdu::*;

impl Initiator {
    pub(super) fn on_response(&mut self, p: &[u8]) -> Step {
        let Ok(peer) = Pairing::parse(p) else { return self.fail(INVALID_PARAMETERS) };
        // A responder without Secure Connections would fall back to legacy
        // pairing, which this host refuses (see the crate's notes).
        if peer.auth_req & auth::SC == 0 {
            return self.fail(AUTHENTICATION_REQUIREMENTS);
        }
        // The key size is the smaller of the two (Vol 3, Part H, 2.3.4);
        // a shorter key is weaker than the link's own crypto.
        if peer.max_key_size < self.ours.max_key_size {
            return self.fail(ENCRYPTION_KEY_SIZE);
        }
        // The peer holds OOB data for us, but we gave none out, so the
        // method it would expect cannot run (Vol 3, Part H, 2.3.5.1).
        if peer.oob {
            return self.fail(OOB_NOT_AVAILABLE);
        }
        let mitm = |p: &Pairing| p.auth_req & auth::MITM != 0;
        self.method = select(self.ours.io_cap, mitm(&self.ours), peer.io_cap, mitm(&peer));
        self.bond.authenticated = self.method != Method::JustWorks;
        self.peer = Some(peer);
        self.state = State::AwaitPublicKey;
        let (x, y) = self.keys.public();
        let mut params = [0u8; 64];
        params[..32].copy_from_slice(&le(x));
        params[32..].copy_from_slice(&le(y));
        Step::send(pdu(PAIRING_PUBLIC_KEY, &params))
    }
}
