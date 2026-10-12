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

//! Starting discovery, sending a request, and stopping.

use super::charge::Charge;
use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use super::types::MAX_REQUESTS;
use crate::error::HogpError;
use crate::report::{Keyboard, Locks, Mouse};
use crate::uuids::{HID_SERVICE, PRIMARY_SERVICE};
use alloc::vec::Vec;
use nonos_bt_wire::att;

impl Host {
    /// A host on a bearer that is already encrypted; the step holds the
    /// first request.
    pub fn start(now_ms: u64) -> (Self, Step) {
        let mut host = Self {
            phase: Phase::Services { next: 1 },
            services: Vec::new(),
            decls: Vec::new(),
            sources: Vec::new(),
            modes: Vec::new(),
            outputs: Vec::new(),
            locks: Locks::default(),
            keyboard: Keyboard::default(),
            mouse: Mouse::default(),
            charge: Charge::Idle,
            banks: Vec::new(),
            cells: Vec::new(),
            pending: None,
            sent_ms: now_ms,
            requests: 0,
        };
        let step = host.find_services(1, now_ms);
        (host, step)
    }

    pub(super) fn find_services(&mut self, next: u16, now_ms: u64) -> Step {
        self.phase = Phase::Services { next };
        let value = HID_SERVICE.to_le_bytes();
        self.request(att::find_by_type_value(next, 0xffff, PRIMARY_SERVICE, &value), now_ms)
    }

    pub(super) fn request(&mut self, pdu: Vec<u8>, now_ms: u64) -> Step {
        self.requests += 1;
        if self.requests > MAX_REQUESTS {
            return self.fail(HogpError::TooMany);
        }
        (self.pending, self.sent_ms) = (pdu.first().copied(), now_ms);
        Step::send(pdu)
    }
}
