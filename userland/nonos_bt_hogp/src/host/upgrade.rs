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

//! Report mode, looked at once every service's declarations are read. A
//! keyboard service worth upgrading has its Report Map read whole: a Read,
//! then Read Blobs from the length so far while each answer fills a
//! response at the bearer's ATT_MTU (Vol 3, Part G, 4.8.1 and 4.8.3). A map longer than HIDS 1.0
//! allows leaves the service in boot mode, unread.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use crate::report::map::{self, MAP_MAX};
use nonos_bt_wire::att;

impl Host {
    /// The first service from `from` on worth upgrading, or the CCCDs.
    pub(super) fn upgrade(&mut self, from: usize, now_ms: u64) -> Result<Step, HogpError> {
        let next = self.hids.iter().enumerate().skip(from).find(|(_, h)| h.worth());
        let Some((service, map)) = next.and_then(|(i, h)| Some((i, h.map?))) else {
            self.map.clear();
            // Only now: a service with no boot reports may give the input.
            if self.sources.is_empty() {
                return Err(HogpError::NoHid);
            }
            return self.find_cccd(0, now_ms);
        };
        self.map.clear();
        self.phase = Phase::Map { service };
        Ok(self.request(att::read(map), now_ms))
    }

    pub(super) fn on_map(&mut self, service: usize, p: &[u8], now: u64) -> Result<Step, HogpError> {
        if self.map.len() + p.len() > MAP_MAX {
            return self.upgrade(service + 1, now);
        }
        self.map.extend_from_slice(p);
        // A response shorter than it could be ends the value.
        if p.len() < usize::from(self.mtu) - 1 {
            return self.refs(service, 0, now);
        }
        let map = self.hids.get(service).and_then(|h| h.map).ok_or(HogpError::Protocol)?;
        Ok(self.request(att::read_blob(map, self.map.len() as u16), now))
    }

    /// The map read whole: the service is switched if it can be.
    pub(super) fn take_up(&mut self, service: usize, now: u64) -> Result<Step, HogpError> {
        if let Ok(layout) = map::parse(&core::mem::take(&mut self.map)) {
            self.switch(service, layout);
        }
        self.upgrade(service + 1, now)
    }
}
