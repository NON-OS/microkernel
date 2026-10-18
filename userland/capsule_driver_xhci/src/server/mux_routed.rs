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

//! A routed Address Device across controllers. Its TT hub slot is one of
//! the multiplexer's too, and must be bound on the port's controller; and
//! its whole place is held against that controller's hubs before a slot is
//! enabled for it, so a request naming no place a hub set up has no
//! command run for it, as with one controller.

use alloc::vec::Vec;

use super::mux::Mux;
use super::mux_forward::VSlot;
use crate::server::handlers::address_routed::place_below_hub;

/// Where a routed request carries its speed, route, and TT hub slot.
const SPEED: usize = 2;
const ROUTE: usize = 3;
const TT_SLOT: usize = 7;

impl Mux {
    /// `body`, `ADDRESS_ROUTED_REQUEST_LEN` bytes for root port `lport` of
    /// controller `ctrl`, with its TT hub slot made the controller's own;
    /// `Err` when it names a hub not bound there or a place none has.
    pub(super) fn routed_local(&self, ctrl: usize, lport: u8, body: &[u8]) -> Result<Vec<u8>, ()> {
        let mut local = Vec::from(body);
        if body[TT_SLOT] != 0 {
            match self.vslots[body[TT_SLOT] as usize] {
                VSlot::Bound { ctrl: c, slot } if c as usize == ctrl => local[TT_SLOT] = slot,
                _ => return Err(()),
            }
        }
        let r = &body[ROUTE..ROUTE + 4];
        let route = u32::from_le_bytes([r[0], r[1], r[2], r[3]]);
        let tt = (local[TT_SLOT], local[TT_SLOT + 1]);
        place_below_hub(&self.ctxs[ctrl], lport, body[SPEED], route, tt).ok_or(())?;
        Ok(local)
    }
}
