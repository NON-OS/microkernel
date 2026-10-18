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

//! Key distribution, once the link is encrypted (Vol 3, Part H, 3.6.1).
//! The responder's identity is asked for: its IRK in Identity Information,
//! then its identity address (Vol 3, Part H, 3.6.4, 3.6.5). Ours follows
//! (`identity.rs`).

use super::fields::take;
use super::initiator::Initiator;
use super::state::State;
use super::step::Step;
use crate::error::SmpError;
use crate::pdu::*;

impl Initiator {
    /// The controller reported the link encrypted with the LTK.
    pub fn encrypted(&mut self, now_ms: u64) -> Result<Step, SmpError> {
        if self.state != State::AwaitEncryption {
            return Err(SmpError::Refused);
        }
        if self.expired(now_ms) {
            return Ok(self.time_out());
        }
        self.last_ms = now_ms;
        let ours = self.ours.resp_keys;
        let due = self.peer.map_or(0, |p| p.resp_keys) & ours & keys::ID;
        Ok(self.finish(due))
    }

    pub(super) fn on_irk(&mut self, p: &[u8]) -> Step {
        let Some(irk) = take::<16>(p, 0) else { return self.fail(INVALID_PARAMETERS) };
        // Sent twice, it is out of the order 3.6.1 fixes.
        if self.bond.irk.is_some() {
            return self.fail(UNSPECIFIED_REASON);
        }
        self.bond.irk = Some(be(&irk));
        Step::default()
    }

    pub(super) fn on_identity(&mut self, due: u8, p: &[u8]) -> Step {
        let Some(id) = take::<7>(p, 0) else { return self.fail(INVALID_PARAMETERS) };
        if self.bond.irk.is_none() {
            return self.fail(UNSPECIFIED_REASON);
        }
        let [kind, addr @ ..] = id;
        // Type 0 public, 1 static random; nothing else is an identity.
        if kind > 1 {
            return self.fail(INVALID_PARAMETERS);
        }
        self.bond.identity = Some((kind, addr));
        self.finish(due & !keys::ID)
    }
}
