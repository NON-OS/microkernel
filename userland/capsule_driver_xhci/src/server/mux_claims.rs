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
//! Sharing a device between class drivers (slots/claims.rs): an interface
//! claimed, the shared slot on a port asked for, and a slot given back by
//! one sharer while another keeps it. A slot no interface of was claimed is
//! given back as before.

use nonos_libc::mk_pid_alive;

use super::mux::Mux;
use crate::protocol::{
    encode_response_header, write_status, Request, E_BUSY, E_INVAL, E_NODEV, OP_CLAIM_INTERFACE,
    OP_DISABLE_SLOT, OP_SLOT_OF_PORT, RESP_HDR_LEN, STATUS_LEN,
};
use crate::server::error::reply_with_status;
use crate::server::reply::{send, sender};
use crate::slots::Release;

impl Mux {
    /// True when the request was answered here.
    pub(super) fn claim_ops(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
        let pid = sender();
        match (req.op, body) {
            (OP_CLAIM_INTERFACE, &[slot, port, interface]) => {
                let live = self.addressed(self.virt.real(slot)).is_some();
                let s =
                    if live { self.claims.claim(slot, port, interface, pid) } else { Err(E_INVAL) };
                reply_with_status(tx, req, s.err().unwrap_or(0));
            }
            (OP_SLOT_OF_PORT, &[port]) => match self.claims.on_port(port) {
                Some((slot, held)) => {
                    let plen = (STATUS_LEN + 8) as u32;
                    encode_response_header(tx, req, plen);
                    write_status(&mut tx[RESP_HDR_LEN..], 0);
                    let o = RESP_HDR_LEN + STATUS_LEN;
                    tx[o..o + 4].copy_from_slice(&[slot, 0, 0, 0]);
                    tx[o + 4..o + 8].copy_from_slice(&held.to_le_bytes());
                    send(tx.as_ptr(), RESP_HDR_LEN + plen as usize);
                }
                None => reply_with_status(tx, req, E_NODEV),
            },
            (OP_CLAIM_INTERFACE | OP_SLOT_OF_PORT, _) => reply_with_status(tx, req, E_INVAL),
            (OP_DISABLE_SLOT, &[slot]) => match self.claims.release(slot, pid, |p| mk_pid_alive(p))
            {
                Release::Unclaimed | Release::Last => return false,
                Release::Kept => reply_with_status(tx, req, 0),
                Release::NotHolder => reply_with_status(tx, req, E_BUSY),
            },
            _ => return false,
        }
        true
    }
}
