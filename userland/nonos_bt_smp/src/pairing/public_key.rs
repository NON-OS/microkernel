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

//! The responder's public key (Vol 3, Part H, 3.5.6), the DHKey, and the
//! start of authentication stage 1 for the chosen method.

use super::draw;
use super::fields::take;
use super::initiator::Initiator;
use super::io::Random;
use super::state::State;
use super::step::{Step, User};
use crate::ecdh::PeerKey;
use crate::method::Method;
use crate::pdu::*;

impl Initiator {
    pub(super) fn on_public_key(&mut self, p: &[u8], rng: &mut dyn Random) -> Step {
        let (Some(x), Some(y)) = (take::<32>(p, 0), take::<32>(p, 32)) else {
            return self.fail(INVALID_PARAMETERS);
        };
        let (x, y) = (be(&x), be(&y));
        // Our own key sent back would let a responder answer each passkey
        // round with our own confirm and nonce, so it is refused too.
        if (&x, &y) == self.keys.public() {
            return self.fail(DHKEY_CHECK_FAILED);
        }
        // A key off the curve, or the debug key, ends pairing with DHKey
        // Check Failed, as the 2018 erratum asks for an invalid key.
        let Ok(peer) = PeerKey::new(&x, &y) else { return self.fail(DHKEY_CHECK_FAILED) };
        let Ok(dh_key) = self.keys.dh_key(&peer) else { return self.fail(DHKEY_CHECK_FAILED) };
        (self.peer_x, self.dh_key) = (x, dh_key);
        match self.method {
            // Only the responder commits first in these (2.3.5.6.2).
            Method::JustWorks | Method::NumericComparison => {
                self.na = draw::nonce(rng);
                self.state = State::AwaitConfirm(0);
                Step::default()
            }
            Method::PasskeyInitiatorDisplays => {
                let Some(k) = draw::passkey(rng) else { return self.fail(UNSPECIFIED_REASON) };
                self.passkey = k;
                let mut step = self.round(0, rng);
                step.user = Some(User::ShowPasskey(k));
                step
            }
            Method::PasskeyResponderDisplays | Method::PasskeyBothInput => {
                self.state = State::AwaitPasskey;
                Step::user(User::EnterPasskey)
            }
        }
    }
}
