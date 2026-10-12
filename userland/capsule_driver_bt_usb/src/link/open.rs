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

//! A session on a new link. Pairing starts at once: with no bond kept
//! yet, there is no key to encrypt with, and the HID service may not be
//! read on a link that is not encrypted (HOGP 1.0, 4.1).

use alloc::collections::VecDeque;

use super::env::Env;
use super::session::{Session, PDU_MAX};
use nonos_bt_smp::funcs::addr56;
use nonos_bt_smp::pairing::{Initiator, REQUEST};
use nonos_bt_smp::SmpError;
use nonos_bt_wire::addr::{AddrType, BdAddr};
use nonos_bt_wire::hci::le_meta::LeConnectionComplete;
use nonos_bt_wire::l2cap::cid::SMP;
use nonos_bt_wire::l2cap::Reassembler;

impl Session {
    /// `ours` is the controller's public address, the one LE Create
    /// Connection used (Own_Address_Type 0).
    pub fn open<E: Env>(
        link: &LeConnectionComplete,
        ours: &BdAddr,
        env: &mut E,
    ) -> Result<Self, SmpError> {
        let a = addr56(AddrType::Public, ours);
        let b = addr56(link.peer_type, &link.peer);
        let now = env.now_ms();
        let (smp, request) = Initiator::start(REQUEST, a, b, env, now)?;
        let mut s = Self {
            handle: link.handle,
            smp: Some(smp),
            hid: None,
            joiner: Reassembler::new(PDU_MAX),
            outbox: VecDeque::new(),
            end: None,
            bond: None,
        };
        s.send(SMP, &request);
        Ok(s)
    }
}
