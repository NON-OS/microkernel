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

//! ACL data from the peer: fragments joined into L2CAP PDUs (Vol 3, Part
//! A, 7.2) and each handed to its fixed channel (Vol 3, Part A, 2.1). Data
//! that does not parse is the peer's fault and costs that PDU only: the
//! protocol above it times out, as it would for one lost on the air.

use super::env::Env;
use super::session::Session;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::hci::parse_acl;
use nonos_bt_wire::l2cap::cid::{ATT, LE_SIGNALING, SMP};
use nonos_bt_wire::l2cap::{answer_central, basic, parse_basic};

impl Session {
    pub(super) fn on_acl<T: Transport, E: Env>(
        &mut self,
        host: &mut Host<T>,
        packet: &[u8],
        env: &mut E,
    ) -> Result<(), HciError> {
        let Ok((head, data)) = parse_acl(packet) else { return Ok(()) };
        if head.handle != self.handle {
            return Ok(());
        }
        self.quiet_since = None;
        let Ok(Some(pdu)) = self.joiner.push(head.boundary, data) else { return Ok(()) };
        let Ok((cid, payload)) = parse_basic(&pdu) else { return Ok(()) };
        match cid {
            SMP => self.on_smp(host, payload, env),
            ATT => {
                self.on_att(payload, env);
                Ok(())
            }
            LE_SIGNALING => self.on_signal(host, payload),
            _ => Ok(()),
        }
    }

    /// A Connection Parameter Update Request this host accepts is applied
    /// at once with LE Connection Update; a refusal is the request's
    /// answer alone (Vol 3, Part A, 4.20 and 4.21).
    fn on_signal<T: Transport>(
        &mut self,
        host: &mut Host<T>,
        frame: &[u8],
    ) -> Result<(), HciError> {
        let Ok(answer) = answer_central(frame) else { return Ok(()) };
        if let Some(reply) = answer.reply.and_then(|r| basic(LE_SIGNALING, &r).ok()) {
            self.outbox.push_back(reply);
        }
        // The answer goes out ahead of the new timing it agrees to.
        self.flush(host)?;
        match answer.update.map(|p| host.update_link(self.handle, &p)) {
            // A refusal keeps the link as it was, and the peripheral may ask
            // again; no answer at all, or a controller reset, is no refusal.
            Some(Err(HciError::Status(..))) | Some(Ok(_)) | None => Ok(()),
            Some(Err(e)) => Err(e),
        }
    }
}
