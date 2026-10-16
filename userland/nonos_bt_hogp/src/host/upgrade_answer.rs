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

//! Responses and errors while report mode is looked at. A response that
//! answers no request still fails discovery, as anywhere; anything else
//! the peripheral gets wrong here only leaves a Report, or its service,
//! in boot mode, which needs none of it.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use nonos_bt_wire::att::{error::*, opcode::*};

impl Host {
    pub(super) fn upgrading(&self) -> bool {
        matches!(self.phase, Phase::Map { .. } | Phase::Refs { .. } | Phase::Ref { .. })
    }

    pub(super) fn on_upgrade(&mut self, op: u8, p: &[u8], now: u64) -> Step {
        let done = match (self.phase, op) {
            (Phase::Map { service }, READ_RSP | READ_BLOB_RSP) => self.on_map(service, p, now),
            (Phase::Refs { service, report, next }, FIND_INFORMATION_RSP) => {
                self.on_refs((service, report, next), p, now)
            }
            (Phase::Ref { service, report }, READ_RSP) => self.on_ref(service, report, p, now),
            _ => Err(HogpError::Protocol),
        };
        done.unwrap_or_else(|e| self.fail(e))
    }

    /// Past the end of a map read whole, Read Blob answers Attribute Not
    /// Long or Invalid Offset (Vol 3, Part F, 3.4.4.5): the map is done.
    pub(super) fn upgrade_error(&mut self, code: u8, now: u64) -> Step {
        let done = match (self.phase, code) {
            (Phase::Map { service }, ATTRIBUTE_NOT_LONG | INVALID_OFFSET)
                if !self.map.is_empty() =>
            {
                self.refs(service, 0, now)
            }
            (Phase::Map { service }, _) => self.upgrade(service + 1, now),
            (Phase::Refs { service, report, .. }, ATTRIBUTE_NOT_FOUND) => {
                self.reference(service, report, now)
            }
            (Phase::Refs { service, report, .. } | Phase::Ref { service, report }, _) => {
                self.pass(service, report, now)
            }
            _ => Err(HogpError::Att(code)),
        };
        done.unwrap_or_else(|e| self.fail(e))
    }
}
