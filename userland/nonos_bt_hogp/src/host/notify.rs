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

//! A Battery Level's CCCD found (Vol 3, Part G, 4.7.1) and written
//! (4.12.3), so later changes arrive without asking. A level with no CCCD
//! has still been read; it is only not kept current.

use super::charge::Charge;
use super::state::Host;
use super::step::Step;
use crate::error::HogpError;
use crate::uuids::{CCCD, NOTIFY_ON};
use nonos_bt_wire::att::{self, response, uuid16};

impl Host {
    pub(super) fn on_cell_descs(
        &mut self,
        cell: usize,
        next: u16,
        p: &[u8],
        now: u64,
    ) -> Result<Step, HogpError> {
        let end = self.cells.get(cell).ok_or(HogpError::Protocol)?.end;
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
                self.charge = Charge::Enable { cell };
                return Ok(self.ask(att::write(handle, &NOTIFY_ON.to_le_bytes(), false), now));
            }
            floor = handle.saturating_add(1);
        }
        if floor > end || floor == 0xffff {
            return Ok(self.read_cell(cell + 1, now));
        }
        self.charge = Charge::Descs { cell, next: floor };
        Ok(self.ask(att::find_information(floor, end), now))
    }
}
