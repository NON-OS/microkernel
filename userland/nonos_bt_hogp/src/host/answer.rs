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

//! A response routed by phase. An Error Response must name the request
//! outstanding; Attribute Not Found ends a discovery step, a security
//! error asks for pairing, and any other error ends discovery.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use nonos_bt_wire::att::{error::*, opcode::*, response};

impl Host {
    pub(super) fn answer(&mut self, op: u8, p: &[u8], now_ms: u64) -> Step {
        if op == ERROR_RSP {
            return match response::error_rsp(p) {
                Ok(e) if Some(e.request) != self.pending => self.fail(HogpError::Protocol),
                Ok(e) => {
                    self.pending = None;
                    self.on_error(e.code, now_ms)
                }
                Err(e) => self.fail(e.into()),
            };
        }
        let done = match (self.phase, op) {
            (Phase::Services { next }, FIND_BY_TYPE_VALUE_RSP) => self.on_services(next, p, now_ms),
            (Phase::Chars { service, next }, READ_BY_TYPE_RSP) => {
                self.on_chars(service, next, p, now_ms)
            }
            (Phase::Descs { source, next }, FIND_INFORMATION_RSP) => {
                self.on_descs(source, next, p, now_ms)
            }
            (Phase::Enable { source }, WRITE_RSP) => Ok(self.enabled(source, now_ms)),
            (Phase::Ready, _) => return self.on_charge(op, p, now_ms),
            _ => Err(HogpError::Protocol),
        };
        done.unwrap_or_else(|e| self.fail(e))
    }

    fn on_error(&mut self, code: u8, now_ms: u64) -> Step {
        // Once ready, only the battery client asks; its errors are its own.
        if self.phase == Phase::Ready {
            return self.charge_error(code, now_ms);
        }
        let security =
            [INSUFFICIENT_AUTHENTICATION, INSUFFICIENT_ENCRYPTION, INSUFFICIENT_KEY_SIZE];
        if security.contains(&code) {
            return self.fail(HogpError::Security(code));
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
