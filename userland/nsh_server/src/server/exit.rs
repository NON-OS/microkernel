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

//! The end of a session that ran its course: the shell's exit status as
//! `EXIT`, then the signed record in `RECORD` parts, the last flagged, then
//! `CLOSE`. The footer says why: the client's `CLOSE`, the reason the host
//! was asked to stop the shell, or the shell exiting on its own.

use super::Server;
use crate::record::EntryKind;
use crate::{Attestation, CallError, EndReason, EscEnd, Event, HostSigner, Rand, Roles};
use nsh_wire::{Expect, Kind};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    /// The shell exited with `status`.
    pub fn exit(&mut self, status: i32, now_ms: u64, rand: Rand<'_>) -> Result<(), CallError> {
        self.running()?;
        let Some(live) = self.live.as_mut() else { return Err(CallError::NotNow) };
        let end = live.stopping.unwrap_or(EndReason::Exit);
        let active = live.active.take();
        if live.pending.take().is_some() {
            self.events.push(Event::EscalationEnded);
        }
        if let Some(esc) = active {
            let _ = self.esc_ended(&esc.hash, EscEnd::SessionEnded, now_ms, rand);
        }
        let _ = self.entry(EntryKind::Exit, now_ms, &status.to_le_bytes(), true, rand);
        if let Err(why) = self.send(Kind::Exit, &status.to_le_bytes(), Expect::ClientSession) {
            self.refuse(why, now_ms, rand);
            return Ok(());
        }
        if !self.close_record(end, now_ms, rand) {
            self.events.push(Event::Closed(end));
            self.end_quietly();
            return Ok(());
        }
        if let Err(why) = self.send_record() {
            self.events.push(Event::Refused(why));
        } else if let Err(why) = self.send(Kind::Close, &[], Expect::ClientSession) {
            self.events.push(Event::Refused(why));
        }
        self.events.push(Event::Closed(end));
        self.end_quietly();
        Ok(())
    }
}
