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

//! Discovering one service's characteristics (Vol 3, Part G, 4.6.1):
//! Read By Type Requests for the characteristic declaration. Each value is
//! the properties, the value handle and the UUID (Vol 3, Part G, 3.3.1).

use super::state::Host;
use super::step::Step;
use super::types::{Decl, MAX_DECLS};
use crate::error::HogpError;
use nonos_bt_wire::att::{response, uuid16};

impl Host {
    pub(super) fn on_chars(
        &mut self,
        service: usize,
        next: u16,
        p: &[u8],
        now_ms: u64,
    ) -> Result<Step, HogpError> {
        let end = self.services.get(service).ok_or(HogpError::Protocol)?.1;
        let (&len, list) = p.split_first().ok_or(HogpError::Protocol)?;
        let mut floor = next;
        for (handle, v) in response::handle_pairs(list, usize::from(len))? {
            let &[props, lo, hi, ref uuid @ ..] = v else { return Err(HogpError::Protocol) };
            let value = u16::from_le_bytes([lo, hi]);
            // The value follows its declaration inside the service, and
            // declarations move forward.
            if handle < floor || value <= handle || value > end {
                return Err(HogpError::Protocol);
            }
            if self.decls.len() == MAX_DECLS {
                return Err(HogpError::TooMany);
            }
            self.decls.push(Decl { handle, props, value, uuid: uuid16(uuid) });
            floor = value.saturating_add(1);
        }
        if floor > end || floor == 0xffff {
            return self.service_done(service, now_ms);
        }
        Ok(self.read_chars(service, floor, now_ms))
    }
}
