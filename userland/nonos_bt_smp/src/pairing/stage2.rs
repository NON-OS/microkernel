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

//! Authentication stage 2 (Vol 3, Part H, 2.3.5.6.5): f5 gives the MacKey
//! and the LTK, and each side proves it holds them with f6 over everything
//! stage 1 agreed on. Na and Nb are the last round's in passkey entry.

use super::fields::{pdu, take};
use super::initiator::Initiator;
use super::state::State;
use super::step::{Outcome, Step};
use crate::funcs::{f5, f6};
use crate::pdu::*;

impl Initiator {
    /// ra and rb: the passkey as a 128-bit value in passkey entry, else 0.
    fn r(&self) -> [u8; 16] {
        let mut r = [0u8; 16];
        if self.is_passkey() {
            r[12..].copy_from_slice(&self.passkey.to_be_bytes());
        }
        r
    }

    /// Ea = f6(MacKey, Na, Nb, rb, IOcapA, A, B) goes to the responder.
    pub(super) fn check(&mut self) -> Step {
        let (mac_key, ltk) = f5(&self.dh_key, &self.na, &self.nb, &self.a, &self.b);
        (self.mac_key, self.bond.ltk) = (mac_key, ltk);
        let io_cap = self.ours.f6_io_cap();
        let ea = f6(&mac_key, &self.na, &self.nb, &self.r(), &io_cap, &self.a, &self.b);
        self.state = State::AwaitDhKeyCheck;
        Step::send(pdu(PAIRING_DHKEY_CHECK, &le(&ea)))
    }

    /// Eb = f6(MacKey, Nb, Na, ra, IOcapB, B, A) must match; then the link
    /// is encrypted with the LTK, the initiator starting it.
    pub(super) fn on_dhkey_check(&mut self, p: &[u8]) -> Step {
        let Some(eb) = take::<16>(p, 0) else { return self.fail(INVALID_PARAMETERS) };
        let Some(peer) = self.peer else { return self.fail(UNSPECIFIED_REASON) };
        let io_cap = peer.f6_io_cap();
        let want = f6(&self.mac_key, &self.nb, &self.na, &self.r(), &io_cap, &self.b, &self.a);
        if be(&eb) != want {
            return self.fail(DHKEY_CHECK_FAILED);
        }
        self.state = State::AwaitEncryption;
        Step::outcome(Outcome::Encrypt(self.bond.ltk))
    }
}
