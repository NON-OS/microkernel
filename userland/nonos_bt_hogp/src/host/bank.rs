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

//! One Battery Service's characteristics (Vol 3, Part G, 4.6.1). The
//! service holds one Battery Level (BAS 1.0, 3); the client keeps the
//! first that can be read, and its last handle, where its descriptors end.

use super::charge::{Cell, Charge};
use super::state::Host;
use super::step::Step;
use super::types::{Decl, MAX_DECLS};
use crate::error::HogpError;
use crate::uuids::{prop, BATTERY_LEVEL, CHARACTERISTIC};
use nonos_bt_wire::att::{self, response, uuid16};

impl Host {
    pub(super) fn bank_chars(&mut self, service: usize, next: u16, now_ms: u64) -> Step {
        self.charge = Charge::Chars { service, next };
        let end = self.banks.get(service).map_or(next, |s| s.1);
        self.ask(att::read_by_type(next, end, CHARACTERISTIC), now_ms)
    }

    pub(super) fn on_bank(
        &mut self,
        service: usize,
        next: u16,
        p: &[u8],
        now: u64,
    ) -> Result<Step, HogpError> {
        let end = self.banks.get(service).ok_or(HogpError::Protocol)?.1;
        let (&len, list) = p.split_first().ok_or(HogpError::Protocol)?;
        let mut floor = next;
        for (handle, v) in response::handle_pairs(list, usize::from(len))? {
            let &[props, lo, hi, ref uuid @ ..] = v else { return Err(HogpError::Protocol) };
            let value = u16::from_le_bytes([lo, hi]);
            if handle < floor || value <= handle || value > end || self.decls.len() == MAX_DECLS {
                return Err(HogpError::Protocol);
            }
            self.decls.push(Decl { handle, props, value, uuid: uuid16(uuid) });
            floor = value.saturating_add(1);
        }
        if floor > end || floor == 0xffff {
            return self.bank_done(service, now);
        }
        Ok(self.bank_chars(service, floor, now))
    }

    pub(super) fn bank_done(&mut self, service: usize, now: u64) -> Result<Step, HogpError> {
        let end = self.banks.get(service).ok_or(HogpError::Protocol)?.1;
        let decls = core::mem::take(&mut self.decls);
        let level = |d: &&Decl| d.uuid == Some(BATTERY_LEVEL) && d.props & prop::READ != 0;
        if let Some(i) = decls.iter().position(|d| level(&d)) {
            let last = decls.get(i + 1).map_or(end, |n| n.handle - 1);
            let d = decls[i];
            self.cells.push(Cell { value: d.value, end: last, props: d.props });
        }
        Ok(match self.banks.get(service + 1) {
            Some(&(start, _)) => self.bank_chars(service + 1, start, now),
            None => self.read_cell(0, now),
        })
    }
}
