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
//! outstanding, and goes to `refused`.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use nonos_bt_wire::att::{opcode::*, response};

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
        if self.upgrading() {
            return self.on_upgrade(op, p, now_ms);
        }
        let done = match (self.phase, op) {
            (Phase::Mtu, EXCHANGE_MTU_RSP) => Ok(self.on_mtu(p, now_ms)),
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
}
