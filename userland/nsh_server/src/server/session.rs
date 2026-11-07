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

//! The frames the client sends once the session runs: terminal input, the
//! window, a signal, an escalation, `CLOSE` and `ABORT`. Each is held to
//! its body's rules. After `CLOSE` the client sends nothing more, and
//! anything it does send is a broken session.

use super::Server;
use crate::{Attestation, EndReason, Event, HostSigner, Rand, Roles, Why};
use nsh_wire::{data, empty, signal, window, Kind, WireError};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub(super) fn session(
        &mut self,
        kind: Kind,
        body: &[u8],
        now_ms: u64,
        rand: Rand<'_>,
    ) -> Result<(), Why> {
        let wire = Why::Wire;
        let closing = self.live.as_ref().is_some_and(|l| l.closing);
        if closing && kind != Kind::Abort {
            return Err(wire(WireError::Order));
        }
        let event = match kind {
            Kind::Input => Event::Input(data(body).map_err(wire)?.into()),
            Kind::Window => {
                let w = window(body).map_err(wire)?;
                Event::Window { cols: w.cols, rows: w.rows }
            }
            Kind::Signal => Event::Signal(signal(body).map_err(wire)?),
            Kind::Escalate => return self.escalate(body, now_ms, rand),
            Kind::Close => {
                empty(body).map_err(wire)?;
                if let Some(l) = self.live.as_mut() {
                    l.closing = true;
                }
                self.stop(EndReason::Close);
                return Ok(());
            }
            Kind::Abort => return self.peer_abort(body),
            _ => return Err(wire(WireError::Order)),
        };
        if let Some(l) = self.live.as_mut() {
            l.last_input_ms = now_ms;
        }
        self.events.push(event);
        Ok(())
    }
}

impl<A, S, R> Server<A, S, R> {
    /// Ask the host to stop the shell, once; the first reason stands.
    pub(super) fn stop(&mut self, why: EndReason) {
        let Some(l) = self.live.as_mut() else { return };
        if l.stopping.is_none() {
            l.stopping = Some(why);
            self.events.push(Event::Stop(why));
        }
    }
}
