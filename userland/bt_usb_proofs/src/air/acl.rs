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

//! ACL data both ways. What the host sends is checked against the LE
//! buffer it was given, joined, and handed to the responder or the
//! peripheral; each packet frees its buffer with Number of Completed
//! Packets (Vol 4, Part E, 7.7.19). What the peer sends is cut into
//! packets of the same length, first ones marked as a controller marks
//! them (Vol 4, Part E, 5.4.2).

use super::state::{Air, HANDLE};
use crate::hci_sim::events::completed;
use nonos_bt_wire::hci::{acl, parse_acl, Boundary};
use nonos_bt_wire::l2cap::cid::{ATT, LE_SIGNALING, SMP};
use nonos_bt_wire::l2cap::{basic, parse_basic};

/// The LE buffer length the HCI proofs' controller reports.
const LE_ACL: usize = 27;

impl Air {
    pub(super) fn take_from_host(&mut self, packet: &[u8]) {
        if self.took_acl() {
            return;
        }
        let (head, data) = parse_acl(packet).expect("a malformed ACL packet");
        assert_eq!(head.handle, HANDLE, "data on a link that does not exist");
        assert!(data.len() <= LE_ACL, "an ACL packet longer than the LE buffer");
        let done = completed(HANDLE, 1);
        match self.lazy_credits {
            _ if self.lies.withhold => {}
            true => self.held.push(done),
            false => self.event(&done),
        }
        let Some(pdu) = self.joiner.push(head.boundary, data).expect("a broken fragment run")
        else {
            return;
        };
        let (cid, payload) = parse_basic(&pdu).expect("a malformed L2CAP PDU");
        match cid {
            SMP => self.smp_from_host(payload),
            ATT => self.att_from_host(payload),
            LE_SIGNALING => self.signals.push(payload.to_vec()),
            other => panic!("the host sent on channel {other:#06x}"),
        }
    }

    pub fn send_to_host(&mut self, cid: u16, payload: &[u8]) {
        let pdu = basic(cid, payload).unwrap();
        let mut b = Boundary::FirstFlushable;
        for part in pdu.chunks(LE_ACL) {
            self.bulk_in.push_back(acl(HANDLE, b, part).unwrap());
            b = Boundary::Continuing;
        }
    }
}
