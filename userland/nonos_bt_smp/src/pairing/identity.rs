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

//! This host's identity, sent once the responder's keys are in: the
//! initiator distributes after the responder (Vol 3, Part H, 3.6.1). With
//! it a bonded peer can resolve the private addresses this host uses
//! (Vol 6, Part B, 1.3.2.3) and tell it from a stranger.

use super::fields::pdu;
use super::initiator::Initiator;
use super::state::State;
use super::step::{Outcome, Step};
use crate::pdu::*;
use alloc::vec::Vec;

/// This host's identity resolving key, most significant octet first, and
/// its identity address: type 0 public or 1 static random, address in wire
/// order (Vol 3, Part H, 3.6.4 and 3.6.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Identity {
    pub irk: [u8; 16],
    pub addr_type: u8,
    pub addr: [u8; 6],
}

impl Initiator {
    /// With `due` of the responder's keys still to come, wait for them;
    /// otherwise send ours, if both sides agreed on them, and finish.
    pub(super) fn finish(&mut self, due: u8) -> Step {
        if due != 0 {
            self.state = State::AwaitKeys(due);
            return Step::default();
        }
        self.state = State::Done;
        let mut step = Step::outcome(Outcome::Paired(self.bond));
        step.send = self.identity_out();
        step
    }

    /// Identity Information then Identity Address Information, in the order
    /// 3.6.1 fixes, when the response asked this host for its identity.
    fn identity_out(&self) -> Vec<Vec<u8>> {
        let agreed = self.peer.map_or(0, |p| p.init_keys) & self.ours.init_keys & keys::ID;
        let Some(me) = self.local.filter(|_| agreed != 0) else { return Vec::new() };
        let mut addr = [0u8; 7];
        addr[0] = me.addr_type;
        addr[1..].copy_from_slice(&me.addr);
        alloc::vec![
            pdu(IDENTITY_INFORMATION, &le(&me.irk)),
            pdu(IDENTITY_ADDRESS_INFORMATION, &addr)
        ]
    }
}
