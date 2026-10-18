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

//! Finding each report's CCCD (Vol 3, Part G, 4.7.1): Find Information
//! Requests over the handles after its value, up to its last handle. The
//! response's format says whether its UUIDs are 16 or 128 bits (Vol 3,
//! Part F, 3.4.3.2).

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use crate::uuids::CCCD;
use nonos_bt_wire::att::{self, response, uuid16};

impl Host {
    pub(super) fn find_cccd(&mut self, source: usize, now_ms: u64) -> Result<Step, HogpError> {
        // A report source's CCCD was found with its Report Reference.
        let left = (source..self.sources.len()).find(|&i| self.sources[i].cccd.is_none());
        let Some(source) = left else { return Ok(self.enable(0, now_ms)) };
        let s = self.sources[source];
        let (next, end) = (s.value.saturating_add(1), s.end);
        if next > end || s.value == 0xffff {
            return Err(HogpError::NoCccd);
        }
        self.phase = Phase::Descs { source, next };
        Ok(self.request(att::find_information(next, end), now_ms))
    }

    pub(super) fn on_descs(
        &mut self,
        source: usize,
        next: u16,
        p: &[u8],
        now_ms: u64,
    ) -> Result<Step, HogpError> {
        let end = self.sources.get(source).ok_or(HogpError::Protocol)?.end;
        let each = match p.first() {
            Some(1) => 4,
            Some(2) => 18,
            _ => return Err(HogpError::Protocol),
        };
        let mut floor = next;
        for (handle, uuid) in response::handle_pairs(&p[1..], each)? {
            if handle < floor || handle > end {
                return Err(HogpError::Protocol);
            }
            if uuid16(uuid) == Some(CCCD) {
                self.sources[source].cccd = Some(handle);
                return self.find_cccd(source + 1, now_ms);
            }
            floor = handle.saturating_add(1);
        }
        if floor > end || floor == 0xffff {
            return Err(HogpError::NoCccd);
        }
        self.phase = Phase::Descs { source, next: floor };
        Ok(self.request(att::find_information(floor, end), now_ms))
    }
}
