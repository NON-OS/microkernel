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

//! Ending a connection. The client is told the one reason code a refusal
//! allows, and only once keys exist to seal it. A session that ran has its
//! record ended and signed, and handed to the host, whatever ended it;
//! then every key and the handshake secret are dropped, which wipes them.

use super::stage::Stage;
use super::Server;
use crate::{Attestation, EndReason, Event, HostSigner, Rand, Roles, Why};
use nsh_wire::Kind;

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// End the connection over `why`, telling the client what the wire
    /// allows.
    pub(super) fn refuse(&mut self, why: Why, now_ms: u64, rand: Rand<'_>) {
        self.events.push(Event::Refused(why));
        if let Some(reason) = why.abort() {
            let peer = self.stage.peer();
            let _ = self.send(Kind::Abort, &reason.code().to_le_bytes(), peer);
        }
        if self.live.is_some() {
            let end = match why {
                Why::Wire(_) | Why::Open | Why::Limit | Why::Aborted(_) => EndReason::Aborted,
                _ => EndReason::Host,
            };
            self.close_record(end, now_ms, rand);
            self.events.push(Event::Closed(end));
        }
        self.end_quietly();
    }

    /// End the connection with nothing more sent, and drop every key.
    pub(super) fn end_quietly(&mut self) {
        self.stage = Stage::Over;
        (self.hs, self.sealer, self.opener, self.admitted) = (None, None, None, None);
        if let Some(mut live) = self.live.take() {
            live.record.wipe();
        }
    }
}
