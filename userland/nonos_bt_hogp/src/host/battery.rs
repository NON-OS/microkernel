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

//! The Battery Service client (BAS 1.0). A HID device carries the service
//! (HOGP 1.0, 3), and its level is how the user learns a keyboard is about
//! to stop. It starts once the HID reports are on, so typing never waits
//! on it, and keeps to the one request outstanding (Vol 3, Part F, 3.3.2).
//! Whatever it cannot use ends it alone: the keyboard goes on working with
//! no level known.

use super::charge::Charge;
use super::state::Host;
use super::step::Step;
use super::types::{MAX_REQUESTS, MAX_SERVICES};
use crate::error::HogpError;
use crate::uuids::{BATTERY_SERVICE, PRIMARY_SERVICE};
use alloc::vec::Vec;
use nonos_bt_wire::att::{self, response};

impl Host {
    pub(super) fn find_banks(&mut self, next: u16, now_ms: u64) -> Step {
        self.charge = Charge::Services { next };
        let value = BATTERY_SERVICE.to_le_bytes();
        self.ask(att::find_by_type_value(next, 0xffff, PRIMARY_SERVICE, &value), now_ms)
    }

    /// A request for the battery client. The bearer's request budget is
    /// shared; when it is spent the battery yields, rather than end HID.
    pub(super) fn ask(&mut self, pdu: Vec<u8>, now_ms: u64) -> Step {
        if self.requests >= MAX_REQUESTS {
            self.charge = Charge::Done;
            return Step::default();
        }
        self.request(pdu, now_ms)
    }

    /// Battery Services found by UUID (Vol 3, Part G, 4.4.2), as for HID.
    /// Instances past the bound are left unread, not refused: a level is a
    /// courtesy, and the first few are the device's own.
    pub(super) fn on_banks(&mut self, next: u16, p: &[u8], now: u64) -> Result<Step, HogpError> {
        let mut floor = next;
        for (start, end) in response::found_ranges(p)? {
            if start < floor || end < start || start == 0 {
                return Err(HogpError::Protocol);
            }
            if self.banks.len() == MAX_SERVICES {
                return Ok(self.banks_done(now));
            }
            self.banks.push((start, end));
            floor = end.saturating_add(1);
        }
        match self.banks.last() {
            Some(&(_, 0xffff)) => Ok(self.banks_done(now)),
            _ => Ok(self.find_banks(floor, now)),
        }
    }
}
