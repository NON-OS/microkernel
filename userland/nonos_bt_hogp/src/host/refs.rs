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

//! Each Report's descriptors (Vol 3, Part G, 4.7.1): its Report Reference,
//! which gives its ID and type (HIDS 1.0, 3.6.2.1), and its CCCD. A Report
//! whose descriptors cannot be read, or that has no Reference, is passed
//! over; a service with none left keeps boot mode.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use crate::uuids::prop;
use nonos_bt_wire::att;

impl Host {
    /// The descriptors of the first Report from `from` on that can notify
    /// or take a Write Command, and has handles after its value.
    pub(super) fn refs(
        &mut self,
        service: usize,
        from: usize,
        now: u64,
    ) -> Result<Step, HogpError> {
        let reps = self.hids.get(service).map_or(&[][..], |h| &h.reports[..]);
        let wanted = |r: &super::hid::Rep| {
            r.props & (prop::NOTIFY | prop::WRITE_WITHOUT_RESPONSE) != 0 && r.value < r.end
        };
        let Some(report) = (from..reps.len()).find(|&i| wanted(&reps[i])) else {
            return self.take_up(service, now);
        };
        let (next, end) = (reps[report].value + 1, reps[report].end);
        self.phase = Phase::Refs { service, report, next };
        Ok(self.request(att::find_information(next, end), now))
    }

    /// The Report Reference, if one was found; then the next Report.
    pub(super) fn reference(
        &mut self,
        service: usize,
        report: usize,
        now: u64,
    ) -> Result<Step, HogpError> {
        let refh = self.hids.get(service).and_then(|h| h.reports.get(report)?.refh);
        let Some(handle) = refh else { return self.pass(service, report, now) };
        self.phase = Phase::Ref { service, report };
        Ok(self.request(att::read(handle), now))
    }

    pub(super) fn pass(
        &mut self,
        service: usize,
        report: usize,
        now: u64,
    ) -> Result<Step, HogpError> {
        self.refs(service, report + 1, now)
    }
}
