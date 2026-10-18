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

//! Sending one L2CAP PDU as ACL packets no longer than the controller
//! takes. On LE the host marks the first packet "first non-automatically-
//! flushable" and the rest "continuing" (Vol 4, Part E, 5.4.2). Each packet
//! takes one controller buffer, held for its link (`held`), until Number
//! of Completed Packets gives it back (Vol 4, Part E, 4.1.1).

use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::acl::acl;
use nonos_bt_wire::hci::boundary::Boundary;

impl<T: Transport> Host<T> {
    /// Send `pdu` on `handle` whole or not at all: `NoCredit` when the
    /// controller has fewer free buffers than the PDU needs packets, so no
    /// PDU is ever left half sent.
    pub fn send_pdu(&mut self, handle: u16, pdu: &[u8]) -> Result<(), HciError> {
        if self.acl_len == 0 {
            return Err(HciError::NoBuffers);
        }
        // An L2CAP PDU always has its four-octet header (Vol 3, Part A, 3.1).
        if pdu.is_empty() {
            return Err(HciError::Wire(nonos_bt_wire::WireError::Short));
        }
        let parts = pdu.chunks(usize::from(self.acl_len));
        if parts.len() > usize::from(self.acl_free) {
            return Err(HciError::NoCredit);
        }
        let mut boundary = Boundary::FirstNonFlushable;
        for part in parts {
            self.t.send_acl(&acl(handle, boundary, part)?)?;
            self.hold(handle);
            boundary = Boundary::Continuing;
        }
        Ok(())
    }
}
