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

//! Authentication stage 1 (Vol 3, Part H, 2.3.5.6.2 and 2.3.5.6.3). For
//! Just Works and numeric comparison the responder commits to Nb, we send
//! Na, and its Nb must open the commitment. Passkey entry repeats that per
//! passkey bit, each side committing to a fresh nonce and the bit.

use super::draw;
use super::fields::{pdu, take};
use super::initiator::{Initiator, ROUNDS};
use super::io::Random;
use super::state::State;
use super::step::{Step, User};
use crate::funcs::{f4, g2, passkey_display};
use crate::method::Method;
use crate::pdu::*;

impl Initiator {
    /// The z of f4: 0 outside passkey entry, else 0x80 with round r's bit.
    fn z(&self, r: u8) -> u8 {
        if self.is_passkey() {
            0x80 | (self.passkey >> r & 1) as u8
        } else {
            0
        }
    }

    /// Passkey round r: Cai = f4(PKax, PKbx, Nai, rai) goes first.
    pub(super) fn round(&mut self, r: u8, rng: &mut dyn Random) -> Step {
        self.na = draw::nonce(rng);
        let ca = f4(self.keys.public().0, &self.peer_x, &self.na, self.z(r));
        self.state = State::AwaitConfirm(r);
        Step::send(pdu(PAIRING_CONFIRM, &le(&ca)))
    }

    pub(super) fn on_confirm(&mut self, r: u8, p: &[u8]) -> Step {
        let Some(cb) = take::<16>(p, 0) else { return self.fail(INVALID_PARAMETERS) };
        self.cb = be(&cb);
        self.state = State::AwaitRandom(r);
        Step::send(pdu(PAIRING_RANDOM, &le(&self.na)))
    }

    pub(super) fn on_random(&mut self, r: u8, p: &[u8], rng: &mut dyn Random) -> Step {
        let Some(nb) = take::<16>(p, 0) else { return self.fail(INVALID_PARAMETERS) };
        let nb = be(&nb);
        if f4(&self.peer_x, self.keys.public().0, &nb, self.z(r)) != self.cb {
            return self.fail(CONFIRM_VALUE_FAILED);
        }
        self.nb = nb;
        match self.method {
            Method::JustWorks => self.check(),
            Method::NumericComparison => {
                self.state = State::AwaitUserConfirm;
                let v = g2(self.keys.public().0, &self.peer_x, &self.na, &self.nb);
                Step::user(User::ConfirmNumber(passkey_display(v)))
            }
            _ if r + 1 < ROUNDS => self.round(r + 1, rng),
            _ => self.check(),
        }
    }
}
