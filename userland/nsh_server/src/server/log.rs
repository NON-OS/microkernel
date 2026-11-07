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

//! Writing to the record from the session: each entry is handed to the
//! host as it is appended, and each checkpoint as soon as the host keys
//! have signed it. A record with no room left stops the shell; the host
//! then ends the session, and the room kept back takes its last entries.

use super::Server;
use crate::record::{EntryKind, Unfit};
use crate::{EscEnd, Event, HostSigner, Rand};

impl<A, S: HostSigner, R> Server<A, S, R> {
    /// Append one entry; `closing` as `Record::append` takes it.
    pub(super) fn entry(
        &mut self,
        kind: EntryKind,
        now_ms: u64,
        body: &[u8],
        closing: bool,
        rand: Rand<'_>,
    ) -> Result<(), Unfit> {
        self.flush(now_ms, closing, rand)?;
        let Some(live) = self.live.as_mut() else { return Ok(()) };
        let ckpt = live.record.append(kind, now_ms, body, closing);
        self.logged(ckpt, rand)
    }

    /// Write the output sent since the last entry.
    pub(super) fn flush(
        &mut self,
        now_ms: u64,
        closing: bool,
        rand: Rand<'_>,
    ) -> Result<(), Unfit> {
        let Some(live) = self.live.as_mut() else { return Ok(()) };
        let ckpt = live.record.flush_output(now_ms, closing);
        self.logged(ckpt, rand)
    }

    /// `ESC_ENDED` for the escalation grant whose hash is `grant`; the
    /// host is told to stop its capsule.
    pub(super) fn esc_ended(
        &mut self,
        grant: &[u8; 64],
        why: EscEnd,
        now_ms: u64,
        rand: Rand<'_>,
    ) -> Result<(), Unfit> {
        let mut body = [0u8; 65];
        body[..64].copy_from_slice(grant);
        body[64] = why.code();
        self.events.push(Event::EscalationEnded);
        self.entry(EntryKind::EscEnded, now_ms, &body, true, rand)
    }
}
