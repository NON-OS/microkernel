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
//! OP_PARK's body (nonos_usb_virt's park.rs reads it): a child driver.usb_hub0
//! addressed below a hub, handed to the host to list as a virtual port.

use crate::place::Place;

pub const PARK_LEN: usize = 12;

/// `slot u8, parent slot u8, parent port u8, speed u8, route u32 le, TT hub
/// slot u8, TT port u8, EP0 size u16 le`.
pub fn park_body(slot: u8, place: &Place, mps0: u16) -> [u8; PARK_LEN] {
    let [r0, r1, r2, r3] = place.route.to_le_bytes();
    let (ts, tp) = place.tt.map_or((0, 0), |t| (t.hub_slot, t.port));
    let [m0, m1] = mps0.to_le_bytes();
    let (parent, port) = place.parent;
    [slot, parent, port, place.speed, r0, r1, r2, r3, ts, tp, m0, m1]
}
