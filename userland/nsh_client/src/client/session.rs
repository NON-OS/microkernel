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

//! The frames the server sends once the session runs, in its ending order:
//! output and escalation answers while the shell runs, then `EXIT`, then
//! the record's parts up to the last, then `CLOSE`. A frame out of that
//! order is refused, and so is a record that would pass its cap.

use super::stage::{Ending, Phase, Stage};
use super::{Client, Event};
use crate::Refusal;
use nsh_wire::{abort, data, empty, exit_status, EscalateResult, Kind, WireError};

const ORDER: Refusal = Refusal::Wire(WireError::Order);

impl<A, S> Client<A, S> {
    pub(super) fn session(&mut self, kind: Kind, body: &[u8]) -> Result<(), Refusal> {
        let Stage::Session(mut p) = self.stage else { return Err(Refusal::Ended) };
        let event = match kind {
            Kind::Output if p.ending == Ending::Running => {
                Event::Output(wire(data(body))?.to_vec())
            }
            Kind::EscalateResult if p.escalating && answerable(&p) => {
                let r = wire(EscalateResult::parse(body))?;
                p.escalating = false;
                Event::Escalated { reason: r.reason, expires_ms: r.expires_ms }
            }
            Kind::Exit if p.ending == Ending::Running => {
                p.ending = Ending::Exited;
                Event::Exit(wire(exit_status(body))?)
            }
            Kind::Record if p.ending != Ending::RecordDone => match self.part(body, &mut p)? {
                Some(e) => e,
                None => {
                    self.stage = Stage::Session(p);
                    return Ok(());
                }
            },
            Kind::Close if p.ending == Ending::RecordDone => {
                wire(empty(body))?;
                self.end();
                self.events.push(Event::Closed);
                return Ok(());
            }
            Kind::Abort => return Err(Refusal::Aborted(wire(abort(body))?)),
            _ => return Err(ORDER),
        };
        self.stage = Stage::Session(p);
        self.events.push(event);
        Ok(())
    }
}

/// An escalation is answered while the shell runs, or after it exits, but
/// never inside the record.
fn answerable(p: &Phase) -> bool {
    matches!(p.ending, Ending::Running | Ending::Exited)
}

pub(super) fn wire<T>(r: Result<T, WireError>) -> Result<T, Refusal> {
    r.map_err(Refusal::Wire)
}
