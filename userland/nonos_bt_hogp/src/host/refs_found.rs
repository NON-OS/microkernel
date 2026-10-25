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

//! A Report's descriptors as found, within its own characteristic, and
//! its Report Reference as read: one octet of Report ID and one of type,
//! 1 input, 2 output, 3 feature (HIDS 1.0, 3.6.2.1).

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use crate::uuids::{CCCD, REPORT_REFERENCE};
use nonos_bt_wire::att::{self, response, uuid16};

type Found = Result<Step, HogpError>;

impl Host {
    pub(super) fn on_refs(&mut self, at: (usize, usize, u16), p: &[u8], now: u64) -> Found {
        let (service, report, next) = at;
        let end = self.hids[service].reports[report].end;
        let each = match p.first() {
            Some(1) => 4,
            Some(2) => 18,
            _ => return self.pass(service, report, now),
        };
        let Ok(pairs) = response::handle_pairs(&p[1..], each) else {
            return self.pass(service, report, now);
        };
        let mut floor = next;
        for (handle, uuid) in pairs {
            if handle < floor || handle > end {
                return self.pass(service, report, now);
            }
            let rep = &mut self.hids[service].reports[report];
            match uuid16(uuid) {
                Some(REPORT_REFERENCE) => rep.refh = Some(handle),
                Some(CCCD) => rep.cccd = Some(handle),
                _ => {}
            }
            floor = handle.saturating_add(1);
        }
        if floor > end || floor == 0xffff {
            return self.reference(service, report, now);
        }
        self.phase = Phase::Refs { service, report, next: floor };
        Ok(self.request(att::find_information(floor, end), now))
    }

    pub(super) fn on_ref(&mut self, service: usize, report: usize, p: &[u8], now: u64) -> Found {
        if let &[id, kind @ 1..=3] = p {
            self.hids[service].reports[report].reference = Some((id, kind));
        }
        self.pass(service, report, now)
    }
}
