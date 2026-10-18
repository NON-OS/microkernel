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

//! Each PDU from the peer, routed by the state pairing is in. A PDU that
//! does not belong there ends pairing (Vol 3, Part H, 3.5.5).

use super::initiator::Initiator;
use super::io::Random;
use super::state::State;
use super::step::{Outcome, Step};
use crate::pdu::*;
use nonos_bt_wire::WireError;

impl Initiator {
    /// One PDU from the SMP channel, at `now_ms`.
    pub fn on_pdu(&mut self, pdu: &[u8], rng: &mut dyn Random, now_ms: u64) -> Step {
        // Once pairing has ended nothing here may answer: after a failure
        // or a timeout, a reply would restart nothing and could be used to
        // probe the host.
        if self.state.finished() {
            return Step::default();
        }
        if self.expired(now_ms) {
            return self.time_out();
        }
        self.last_ms = now_ms;
        let (code, p) = match parse(pdu) {
            Ok(v) => v,
            Err(WireError::Reserved) => return self.fail(COMMAND_NOT_SUPPORTED),
            Err(_) => return self.fail(INVALID_PARAMETERS),
        };
        match (self.state, code) {
            (_, PAIRING_FAILED) => {
                self.state = State::Failed;
                let reason = p.first().copied().unwrap_or(UNSPECIFIED_REASON);
                Step::outcome(Outcome::Failed { reason, by_peer: true })
            }
            // A Security Request asks for the pairing already running, and
            // keypress notifications were not asked for; neither changes it.
            (_, SECURITY_REQUEST | KEYPRESS_NOTIFICATION) => Step::default(),
            (State::AwaitResponse, PAIRING_RESPONSE) => self.on_response(p),
            (State::AwaitPublicKey, PAIRING_PUBLIC_KEY) => self.on_public_key(p, rng),
            (State::AwaitConfirm(r), PAIRING_CONFIRM) => self.on_confirm(r, p),
            (State::AwaitRandom(r), PAIRING_RANDOM) => self.on_random(r, p, rng),
            (State::AwaitDhKeyCheck, PAIRING_DHKEY_CHECK) => self.on_dhkey_check(p),
            (State::AwaitKeys(_), IDENTITY_INFORMATION) => self.on_irk(p),
            (State::AwaitKeys(due), IDENTITY_ADDRESS_INFORMATION) => self.on_identity(due, p),
            _ => self.fail(UNSPECIFIED_REASON),
        }
    }

    /// Whether the SMP timer has run out at `now_ms`; if so pairing has
    /// failed, and the step says so. Nothing is sent (Vol 3, Part H, 3.4).
    pub fn poll(&mut self, now_ms: u64) -> Step {
        if self.expired(now_ms) {
            self.time_out()
        } else {
            Step::default()
        }
    }
}
