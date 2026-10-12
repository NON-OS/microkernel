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

//! One SCI: the fixed events first, then each pending runtime GPE through
//! its method or its native handler (ACPI 6.5, 5.6.4.1). An edge event's
//! status is cleared before it is served, so a second edge during the
//! method is not lost; a level event's after, once the method has quieted
//! the source. Each GPE is disabled while it is served. A level GPE whose
//! method fails stays disabled, since its source is still asserting and
//! would fire forever.

use super::dispatch::Dispatch;
use super::error::EventsError;
use super::handlers::Trigger;
use super::io::EventIo;
use super::report::SciReport;
use super::state::Events;
use crate::exec::Fault;

impl Events {
    pub fn on_sci(
        &mut self,
        io: &mut impl EventIo,
        run: &mut dyn FnMut(Dispatch) -> Result<(), Fault>,
    ) -> Result<SciReport, EventsError> {
        let mut report = SciReport::default();
        let fixed = self.pm1.status(io).ok_or(EventsError::Io)? & self.fixed;
        if fixed != 0 && !self.pm1.clear(io, fixed) {
            return Err(EventsError::Io);
        }
        report.fixed = fixed;
        for n in self.pending(io)? {
            let what = match (self.native.contains(&n), self.handlers.get(&n)) {
                (true, _) => (Dispatch::Native(n), Trigger::Edge),
                (false, Some(h)) => (Dispatch::Method(h.method), h.trigger),
                (false, None) => continue,
            };
            let Some(b) = self.block_of(n).copied() else { continue };
            let edge = what.1 == Trigger::Edge;
            if !b.set_enabled(io, n, false) || (edge && !b.clear(io, n)) {
                return Err(EventsError::Io);
            }
            let result = run(what.0);
            if !edge && !b.clear(io, n) {
                return Err(EventsError::Io);
            }
            if result.is_ok() || edge {
                b.set_enabled(io, n, true).then_some(()).ok_or(EventsError::Io)?;
            } else {
                self.armed.remove(&n);
                report.disabled.push(n);
            }
            report.ran.push((n, result));
        }
        Ok(report)
    }
}
