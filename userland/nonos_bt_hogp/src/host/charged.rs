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

//! The battery client's responses and errors. Any it cannot use ends it,
//! never the HID host: a peer that fumbles its battery still types. Not
//! Found ends a search as it does for HID; a refused read or CCCD write
//! passes to the next level, since a level may need more security than
//! the HID reports and the others may not.

use super::charge::Charge;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use nonos_bt_wire::att::{error::ATTRIBUTE_NOT_FOUND, opcode::*};

impl Host {
    pub(super) fn on_charge(&mut self, op: u8, p: &[u8], now: u64) -> Step {
        let done = match (self.charge, op) {
            (Charge::Services { next }, FIND_BY_TYPE_VALUE_RSP) => self.on_banks(next, p, now),
            (Charge::Chars { service, next }, READ_BY_TYPE_RSP) => {
                self.on_bank(service, next, p, now)
            }
            (Charge::Read { cell }, READ_RSP) => Ok(self.on_read(cell, p, now)),
            (Charge::Descs { cell, next }, FIND_INFORMATION_RSP) => {
                self.on_cell_descs(cell, next, p, now)
            }
            (Charge::Enable { cell }, WRITE_RSP) => Ok(self.read_cell(cell + 1, now)),
            _ => Err(HogpError::Protocol),
        };
        done.unwrap_or_else(|_| self.flat())
    }

    pub(super) fn charge_error(&mut self, code: u8, now: u64) -> Step {
        let done = match (self.charge, code) {
            (Charge::Services { .. }, ATTRIBUTE_NOT_FOUND) => Ok(self.banks_done(now)),
            (Charge::Chars { service, .. }, ATTRIBUTE_NOT_FOUND) => self.bank_done(service, now),
            (Charge::Read { cell } | Charge::Descs { cell, .. } | Charge::Enable { cell }, _) => {
                Ok(self.read_cell(cell + 1, now))
            }
            _ => Err(HogpError::Att(code)),
        };
        done.unwrap_or_else(|_| self.flat())
    }

    pub(super) fn banks_done(&mut self, now_ms: u64) -> Step {
        match self.banks.first() {
            Some(&(start, _)) => self.bank_chars(0, start, now_ms),
            None => self.read_cell(0, now_ms),
        }
    }

    fn flat(&mut self) -> Step {
        self.charge = Charge::Done;
        Step::default()
    }
}
