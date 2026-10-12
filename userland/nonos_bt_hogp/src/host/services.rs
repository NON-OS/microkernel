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

//! Discovering the HID services by their UUID (Vol 3, Part G, 4.4.2):
//! Find By Type Value Requests from handle 1 on, each starting past the
//! last group found, until Attribute Not Found or the last handle.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use super::types::MAX_SERVICES;
use crate::error::HogpError;
use crate::uuids::CHARACTERISTIC;
use nonos_bt_wire::att::{self, response};

impl Host {
    pub(super) fn on_services(
        &mut self,
        next: u16,
        p: &[u8],
        now_ms: u64,
    ) -> Result<Step, HogpError> {
        let mut floor = next;
        for (start, end) in response::found_ranges(p)? {
            // Groups must lie ahead of the request's start and of each
            // other, so every request moves forward and discovery ends.
            if start < floor || end < start || start == 0 {
                return Err(HogpError::Protocol);
            }
            if self.services.len() == MAX_SERVICES {
                return Err(HogpError::TooMany);
            }
            self.services.push((start, end));
            floor = end.saturating_add(1);
        }
        match self.services.last() {
            Some(&(_, 0xffff)) => self.services_done(now_ms),
            _ => Ok(self.find_services(floor, now_ms)),
        }
    }

    pub(super) fn services_done(&mut self, now_ms: u64) -> Result<Step, HogpError> {
        let &(start, _) = self.services.first().ok_or(HogpError::NoHid)?;
        Ok(self.read_chars(0, start, now_ms))
    }

    pub(super) fn read_chars(&mut self, service: usize, next: u16, now_ms: u64) -> Step {
        self.phase = Phase::Chars { service, next };
        let end = self.services.get(service).map_or(next, |s| s.1);
        self.request(att::read_by_type(next, end, CHARACTERISTIC), now_ms)
    }
}
