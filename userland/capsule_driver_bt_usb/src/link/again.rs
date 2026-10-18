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

//! A link to a bonded peer: encrypted with the kept LTK, EDIV and Rand
//! zero as LE Secure Connections leaves them (Vol 3, Part H, 2.4.4), with
//! no pairing. A peer that answers PIN or Key Missing (Vol 1, Part F, 2)
//! has dropped its side of the bond; ours is forgotten, so the next link
//! pairs afresh rather than offer a key the peer will never take.

use super::env::Env;
use super::find::Peer;
use super::session::Session;
use nonos_bt_bond::BondRecord;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_smp::pairing::Bond;
use nonos_bt_wire::hci::le_meta::LeConnectionComplete;

const KEY_MISSING: u8 = 0x06;

impl Session {
    pub fn resume<T: Transport, E: Env>(
        host: &mut Host<T>,
        link: &LeConnectionComplete,
        record: BondRecord,
        env: &mut E,
    ) -> Result<Self, HciError> {
        let mut s = Self::new(link, Peer { kind: link.peer_type, addr: link.peer });
        match host.encrypt(link.handle, &record.ltk) {
            Ok(()) => {
                let identity = Some((record.identity_type, record.identity));
                let (ltk, irk, authenticated) = (record.ltk, record.irk, record.authenticated);
                s.bond = Some(Bond { ltk, irk, identity, authenticated });
                env.say(&[b"encrypted with the kept bond"]);
                env.linked(record.identity_type, record.identity, false);
                s.start_hid(env);
            }
            Err(HciError::Transport(e)) => return Err(HciError::Transport(e)),
            Err(HciError::Status(_, KEY_MISSING)) => {
                env.forget(&record);
                s.give_up("the peer no longer holds the bond");
            }
            Err(_) => s.give_up("encryption with the kept bond failed"),
        }
        Ok(s)
    }
}
