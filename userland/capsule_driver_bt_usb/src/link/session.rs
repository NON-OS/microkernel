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

//! One link to a HID device: pairing on the SMP channel, then HID over
//! GATT on the ATT channel, with the L2CAP PDUs that wait for controller
//! buffers.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::end::End;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_hogp::host::Host as Hid;
use nonos_bt_smp::pairing::{Bond, Initiator};
use nonos_bt_wire::l2cap::{basic, Reassembler};

/// The longest L2CAP PDU kept while it is joined: far above the 69
/// octets of the largest SMP PDU with its header (Vol 3, Part H, 3.5.6),
/// and a bound on a peer that states any length it likes.
pub(super) const PDU_MAX: usize = 512;

pub struct Session {
    pub handle: u16,
    pub(super) smp: Option<Initiator>,
    pub(super) hid: Option<Hid>,
    pub(super) joiner: Reassembler,
    /// Whole PDUs waiting for the controller to free buffers.
    pub(super) outbox: VecDeque<Vec<u8>>,
    pub(super) end: Option<End>,
    /// The keys pairing gave; the key store keeps them once it exists.
    pub bond: Option<Bond>,
}

impl Session {
    /// Queue `payload` for the fixed channel `cid` (Vol 3, Part A, 3.1).
    pub(super) fn send(&mut self, cid: u16, payload: &[u8]) {
        match basic(cid, payload) {
            Ok(frame) => self.outbox.push_back(frame),
            Err(_) => self.give_up("an L2CAP PDU too long to send"),
        }
    }

    /// The first reason this host gives for ending the link stands.
    pub(super) fn give_up(&mut self, why: &'static str) {
        self.end.get_or_insert(End::GaveUp(why));
    }

    /// Send what waits while the controller has buffers; a PDU that does
    /// not fit waits whole for the next turn (`Host::send_pdu`).
    pub(super) fn flush<T: Transport>(&mut self, host: &mut Host<T>) -> Result<(), HciError> {
        while let Some(frame) = self.outbox.front() {
            match host.send_pdu(self.handle, frame) {
                Ok(()) => drop(self.outbox.pop_front()),
                Err(HciError::NoCredit) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
