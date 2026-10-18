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

//! An Error Response to discovery: Attribute Not Found ends a discovery
//! step, any error to the MTU exchange keeps the default ATT_MTU, a
//! security error asks for pairing, an error while report mode is
//! being looked at leaves boot mode, and any other error ends discovery.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use nonos_bt_wire::att::error::*;

impl Host {
    pub(super) fn on_error(&mut self, code: u8, now_ms: u64) -> Step {
        // Once ready, only the battery client asks; its errors are its own.
        if self.phase == Phase::Ready {
            return self.charge_error(code, now_ms);
        }
        // A server may not take the exchange; the default ATT_MTU holds.
        if self.phase == Phase::Mtu {
            return self.find_services(1, now_ms);
        }
        let security =
            [INSUFFICIENT_AUTHENTICATION, INSUFFICIENT_ENCRYPTION, INSUFFICIENT_KEY_SIZE];
        if security.contains(&code) {
            return self.fail(HogpError::Security(code));
        }
        if self.upgrading() {
            return self.upgrade_error(code, now_ms);
        }
        let done = match (self.phase, code) {
            (Phase::Services { .. }, ATTRIBUTE_NOT_FOUND) => self.services_done(now_ms),
            (Phase::Chars { service, .. }, ATTRIBUTE_NOT_FOUND) => {
                self.service_done(service, now_ms)
            }
            (Phase::Descs { .. }, ATTRIBUTE_NOT_FOUND) => Err(HogpError::NoCccd),
            _ => Err(HogpError::Att(code)),
        };
        done.unwrap_or_else(|e| self.fail(e))
    }
}
