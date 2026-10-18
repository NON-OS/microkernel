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

//! The responder's PDUs, and its side of stage 1. It checks the
//! initiator's commitments itself, failing pairing as a real responder
//! would.

use super::peer::Peer;
use nonos_bt_smp::funcs::f4;
use nonos_bt_smp::method::Method;
use nonos_bt_smp::pdu::*;

impl Peer {
    pub(super) fn passkey_entry(&self) -> bool {
        !matches!(self.method, Method::JustWorks | Method::NumericComparison)
    }

    pub(super) fn z(&self) -> u8 {
        if self.passkey_entry() {
            0x80 | (self.passkey >> self.round & 1) as u8
        } else {
            0
        }
    }

    pub fn on(&mut self, pdu: &[u8]) -> Vec<Vec<u8>> {
        let (code, p) = parse(pdu).unwrap();
        match code {
            PAIRING_REQUEST => self.on_request(p),
            PAIRING_PUBLIC_KEY => self.on_public_key(p),
            PAIRING_CONFIRM => {
                self.ca = be(&p.try_into().unwrap());
                vec![self.commit()]
            }
            PAIRING_RANDOM => {
                self.na = be(&p.try_into().unwrap());
                let pkb = *self.keys.public().0;
                if self.passkey_entry() && f4(&self.pka, &pkb, &self.na, self.z()) != self.ca {
                    return vec![vec![PAIRING_FAILED, CONFIRM_VALUE_FAILED]];
                }
                self.round += 1;
                vec![frame(PAIRING_RANDOM, &le(&self.nb)).unwrap()]
            }
            PAIRING_DHKEY_CHECK => self.on_check(&be(&p.try_into().unwrap())),
            _ => Vec::new(),
        }
    }
}
