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
//! its method (ACPI 6.5, 5.6.4.1). An edge event's status is cleared before
//! its _Exx runs, so a second edge during the method is not lost; a level
//! event's after its _Lxx, once the method has quieted the source. Each GPE
//! is disabled while its method runs. A level GPE whose method fails stays
//! disabled, since its source is still asserting and would fire forever.

use super::error::EventsError;
use super::handlers::Trigger;
use super::io::EventIo;
use super::report::SciReport;
use super::state::Events;
use crate::exec::Fault;
use crate::namespace::Id;

impl Events {
    pub fn on_sci(
        &mut self,
        io: &mut impl EventIo,
        run: &mut dyn FnMut(Id) -> Result<(), Fault>,
    ) -> Result<SciReport, EventsError> {
        let mut report = SciReport::default();
        let fixed = self.pm1.status(io).ok_or(EventsError::Io)? & self.fixed;
        if fixed != 0 && !self.pm1.clear(io, fixed) {
            return Err(EventsError::Io);
        }
        report.fixed = fixed;
        for n in self.pending(io)? {
            let (Some(b), Some(h)) = (self.block_of(n).copied(), self.handlers.get(&n).copied())
            else {
                continue;
            };
            let edge = h.trigger == Trigger::Edge;
            if !b.set_enabled(io, n, false) || (edge && !b.clear(io, n)) {
                return Err(EventsError::Io);
            }
            let result = run(h.method);
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
