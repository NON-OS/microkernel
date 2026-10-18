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
//! method fails on its own stays disabled, since its source is still
//! asserting and would fire forever; so does any GPE past the storm bound
//! (`storm`). One the budget cut short is armed again, or after `CUTS` cuts
//! in a row left off for a back-off window (`storm_backoff`, `rearm`).

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
        let mut report = SciReport { fixed: self.take_fixed(io)?, ..SciReport::default() };
        let now = io.now_ms();
        for n in self.pending(io)? {
            let what = match (self.native.contains(&n), self.handlers.get(&n)) {
                (true, _) => (Dispatch::Native(n), Trigger::Edge),
                (false, Some(h)) => (Dispatch::Method(h.method), h.trigger),
                (false, None) => continue,
            };
            let Some(b) = self.block_of(n).copied() else { continue };
            if self.storms.hit(n, now) {
                self.quiet(io, n)?;
                report.stormed.push(n);
                continue;
            }
            let edge = what.1 == Trigger::Edge;
            if !b.set_enabled(io, n, false) || (edge && !b.clear(io, n)) {
                return Err(EventsError::Io);
            }
            let result = run(what.0);
            if !edge && !b.clear(io, n) {
                return Err(EventsError::Io);
            }
            let cut = result.as_ref().is_err_and(Fault::is_budget);
            let off = self.storms.after_run(n, result.is_ok(), cut, now);
            if (result.is_ok() || edge || cut) && off.is_none() {
                b.set_enabled(io, n, true).then_some(()).ok_or(EventsError::Io)?;
            } else {
                self.armed.remove(&n);
                report.disabled.push((n, off));
            }
            report.ran.push((n, result));
        }
        Ok(report)
    }
}
