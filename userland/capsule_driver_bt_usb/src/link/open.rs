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

//! A session on a new link to a peer with no bond. Pairing starts at once:
//! there is no key to encrypt with, and the HID service may not be read on
//! a link that is not encrypted (HOGP 1.0, 4.1).

use alloc::collections::VecDeque;

use super::env::Env;
use super::find::Peer;
use super::session::{Session, PDU_MAX};
use super::us::Us;
use nonos_bt_smp::funcs::addr56;
use nonos_bt_smp::pairing::{Initiator, REQUEST};
use nonos_bt_smp::pdu::{keys, Pairing};
use nonos_bt_smp::SmpError;
use nonos_bt_wire::hci::le_meta::LeConnectionComplete;
use nonos_bt_wire::l2cap::cid::SMP;
use nonos_bt_wire::l2cap::Reassembler;

impl Session {
    /// Pairing as `us`. With an identity to give, the request offers it
    /// (init_keys ID, Vol 3, Part H, 3.6.1), so the peer can resolve this
    /// host's private addresses from then on.
    pub fn open<E: Env>(
        link: &LeConnectionComplete,
        us: &Us,
        env: &mut E,
    ) -> Result<Self, SmpError> {
        let a = addr56(us.kind, &us.addr);
        let b = addr56(link.peer_type, &link.peer);
        let init_keys = if us.identity.is_some() { keys::ID } else { 0 };
        let request = Pairing { init_keys, ..REQUEST };
        let now = env.now_ms();
        let (smp, pdu) = Initiator::start_private(request, us.identity, a, b, env, now)?;
        let mut s = Self::new(link, Peer { kind: link.peer_type, addr: link.peer });
        s.smp = Some(smp);
        s.send(SMP, &pdu);
        Ok(s)
    }

    pub(super) fn new(link: &LeConnectionComplete, peer: Peer) -> Self {
        Self {
            handle: link.handle,
            smp: None,
            hid: None,
            joiner: Reassembler::new(PDU_MAX),
            outbox: VecDeque::new(),
            end: None,
            bond: None,
            peer,
        }
    }
}
