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

//! The record's end: its closing entries, the footer, and the host keys'
//! signature, or, if they fail, the record handed over unsigned.

use super::Server;
use crate::{Attestation, EndReason, EscEnd, Event, HostSigner, Rand, Roles, Why};
use nsh_wire::HybridSig;

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// Write the record's closing entries and its footer, sign it, and
    /// hand the host the rest of it. A record the host keys could not sign
    /// is handed over without its signature: its checkpoints still cover
    /// it as far as they reach.
    pub(super) fn close_record(&mut self, end: EndReason, now_ms: u64, rand: Rand<'_>) -> bool {
        let _ = self.flush(now_ms, true, rand);
        if let Some(esc) = self.live.as_mut().and_then(|l| l.active.take()) {
            let _ = self.esc_ended(&esc.hash, EscEnd::SessionEnded, now_ms, rand);
        }
        let Some(live) = self.live.as_mut() else { return false };
        let digest = live.record.close(now_ms, end);
        let signed = match self.signer.sign(&digest, rand) {
            Some(sig) => {
                live.record.seal(&HybridSig { ed: &sig.ed, ml: &sig.ml });
                true
            }
            None => false,
        };
        self.events.push(Event::Record(live.record.take_new()));
        if !signed {
            self.events.push(Event::Refused(Why::Sign));
        }
        signed
    }
}
