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

//! The host's answer to an admitted escalation. `escalation_begun` means
//! the second session capsule runs with the escalation's capabilities: the
//! record names them and the client is told when they end.
//! `escalation_failed` means the host could not start it; a host that
//! cannot keep what it admitted ends the session with `ABORT(BUSY)`.

use super::Server;
use crate::record::EntryKind;
use crate::{Attestation, CallError, HostSigner, Rand, Roles, Why};
use alloc::vec::Vec;
use nsh_wire::{CapNames, EscalateResult, Reason, Writer};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub fn escalation_begun(&mut self, now_ms: u64, rand: Rand<'_>) -> Result<(), CallError> {
        self.running()?;
        let Some(live) = self.live.as_mut() else { return Err(CallError::NotNow) };
        let esc = live.pending.take().ok_or(CallError::NotNow)?;
        let names = esc.caps.names();
        let names: Vec<&[u8]> = names.iter().map(|n| n.as_bytes()).collect();
        let mut body = alloc::vec![0u8; 64 + 8 + 1 + names.len() * 33];
        let mut w = Writer::new(&mut body);
        let written = w.put(&esc.hash).and_then(|_| w.u64(esc.ends_ms));
        let written = written.and_then(|_| CapNames::write(&mut w, &names)).map(|_| w.done());
        let ends_ms = esc.ends_ms;
        live.active = Some(esc);
        let n = match written {
            Ok(n) => n,
            Err(e) => {
                self.refuse(Why::Wire(e), now_ms, rand);
                return Ok(());
            }
        };
        body.truncate(n);
        let _ = self.entry(EntryKind::EscGranted, now_ms, &body, false, rand);
        let answer = EscalateResult { reason: Reason::Ok, expires_ms: ends_ms };
        if let Err(why) = self.answer(answer) {
            self.refuse(why, now_ms, rand);
        }
        Ok(())
    }

    pub fn escalation_failed(&mut self, now_ms: u64, rand: Rand<'_>) -> Result<(), CallError> {
        self.running()?;
        let pending = self.live.as_mut().and_then(|l| l.pending.take());
        if pending.is_none() {
            return Err(CallError::NotNow);
        }
        self.refuse(Why::Denied, now_ms, rand);
        Ok(())
    }
}
