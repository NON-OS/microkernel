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

//! What a finished pairing leaves: the bond kept, so the peer is found and
//! the link encrypted again next time without pairing (Vol 3, Part C,
//! 9.4.4), then the HID service set up on the link it encrypted.

use super::env::Env;
use super::session::Session;
use nonos_bt_bond::{BondError, BondRecord};
use nonos_bt_smp::pairing::Bond;

impl Session {
    pub(super) fn paired<E: Env>(&mut self, bond: Bond, env: &mut E) {
        (self.bond, self.smp) = (Some(bond), None);
        env.say(&[b"paired"]);
        match BondRecord::from_pairing(&bond, self.peer.kind, &self.peer.addr) {
            Ok(record) => {
                env.linked(record.identity_type, record.identity, true);
                env.keep(record);
            }
            // It connected from a private address and sent no IRK: it could
            // never be found again, so it pairs afresh each time.
            Err(BondError::NoIdentity) => env.say(&[b"bond not kept: the peer gave no identity"]),
            Err(_) => env.say(&[b"bond not kept: the peer's identity is malformed"]),
        }
        self.start_hid(env);
    }
}
