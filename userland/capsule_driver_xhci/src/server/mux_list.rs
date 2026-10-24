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
//! Giving back the slot of a parked child or of its claimant, and the
//! port list with the virtual ports after the root ports (nonos_usb_virt's
//! report.rs has the bytes).

use super::mux::{status, Mux};
use crate::protocol::{encode_response_header, Request, OP_DISABLE_SLOT, RESP_HDR_LEN, STATUS_LEN};
use crate::server::error::reply_with_status;
use crate::server::reply::{captured, send, vouch};
use nonos_usb_virt::{ext, port_entry, Disable};

impl Mux {
    /// Slot id `s` disabled as any slot is, its reply kept back.
    fn quietly(&mut self, req: &Request, s: u8, tx: &mut [u8]) -> Option<i32> {
        // A parked child's slot is driver.usb_hub0's, given back for it.
        vouch();
        let off = Request { op: OP_DISABLE_SLOT, payload_len: 1, ..*req };
        let len = captured(|| self.inner(&off, &[s], tx));
        status(tx, len)
    }

    pub(super) fn give_back(&mut self, req: &Request, s: u8, tx: &mut [u8]) {
        let done = match self.virt.disable(s) {
            Disable::Detached => Some(0),
            Disable::Release | Disable::Detach(_) | Disable::Plain => self.quietly(req, s, tx),
            Disable::ReleaseGone(parked) => {
                let own = self.quietly(req, s, tx);
                self.quietly(req, parked, tx).and(own)
            }
        };
        reply_with_status(tx, req, done.unwrap_or(-5));
    }

    pub(super) fn list(&mut self, req: &Request, tx: &mut [u8]) {
        let len = captured(|| self.inner(req, &[], tx));
        if status(tx, len) != Some(0) {
            return send(tx.as_ptr(), len);
        }
        let head = RESP_HDR_LEN + STATUS_LEN;
        let (mut end, mut count) = (len, tx[head] as usize);
        let mut virtual_n = 0u8;
        for (vport, p) in self.virt.entries() {
            let held = self.at(p.slot).map_or(1, |(c, l)| self.ctxs[c].driver.slots.held(l));
            tx[end..end + 8].copy_from_slice(&port_entry(vport, p, held));
            (end, count, virtual_n) = (end + 8, count + 1, virtual_n + 1);
        }
        for (vport, p) in self.virt.entries() {
            tx[end..end + 8].copy_from_slice(&ext(vport, p));
            end += 8;
        }
        self.virt.reported();
        (tx[head], tx[head + 1]) = (count as u8, virtual_n);
        encode_response_header(tx, req, (end - RESP_HDR_LEN) as u32);
        send(tx.as_ptr(), end);
    }
}
