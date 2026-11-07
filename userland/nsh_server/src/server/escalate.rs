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

//! `ESCALATE`, checked by the role table against the session's own login:
//! one at a time, the parent is the base grant, a window of at most an
//! hour, within the role's escalation ceiling, with the approvals the role
//! demands. A refusal is answered `ESCALATE_RESULT` with its code, written
//! to the record, and the session goes on; an escalation that passes is
//! handed to the host to start, and answered once it has.

use super::Server;
use crate::record::EntryKind;
use crate::{Attestation, Event, HostSigner, Rand, Roles, Why};
use nsh_roles::{escalate, Host};
use nsh_wire::{EscalateResult, Expect, Kind, Reason};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub(super) fn escalate(&mut self, body: &[u8], now_ms: u64, rand: Rand<'_>) -> Result<(), Why> {
        let g = nsh_wire::escalate(body).map_err(Why::Wire)?;
        let hash = nsh_roles::grant_hash(&g);
        let Some(live) = self.live.as_ref() else { return Err(Why::Denied) };
        if live.stopping.is_some() {
            return Ok(());
        }
        let busy = live.active.is_some() || live.pending.is_some();
        let checked = self.roles.with(|policy, list| match Host::new(policy, list, now_ms) {
            Ok(host) => escalate(&host, &live.login, busy, &g).map_err(Why::Escalation),
            Err(e) => Err(Why::Policy(e)),
        });
        let checked = checked.unwrap_or_else(|e| Err(Why::Policy(e)));
        let why = match checked {
            Ok(esc) => {
                let (caps, ends_ms) = (esc.caps, esc.ends_ms);
                if let Some(l) = self.live.as_mut() {
                    l.pending = Some(esc);
                }
                self.events.push(Event::Escalate { caps, ends_ms });
                return Ok(());
            }
            Err(why) => why,
        };
        let reason = match why {
            Why::Escalation(r) => r.reason(),
            _ => Reason::RevocationStale,
        };
        if !reason.refuses_escalation() {
            return Err(why);
        }
        self.events.push(Event::Refused(why));
        let mut rec = [0u8; 66];
        rec[..2].copy_from_slice(&reason.code().to_le_bytes());
        rec[2..].copy_from_slice(&hash);
        let _ = self.entry(EntryKind::EscRefused, now_ms, &rec, false, rand);
        self.answer(EscalateResult { reason, expires_ms: 0 })
    }

    pub(super) fn answer(&mut self, r: EscalateResult) -> Result<(), Why> {
        let mut body = [0u8; 10];
        let n = r.encode(&mut body).map_err(Why::Wire)?;
        self.send(Kind::EscalateResult, &body[..n], Expect::ClientSession)
    }
}
