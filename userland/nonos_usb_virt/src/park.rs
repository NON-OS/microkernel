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
//! OP_PARK (0x22), from driver.usb_hub0 only, once it has addressed a
//! child with OP_ADDRESS_ROUTED: `slot u8, parent slot u8, parent port u8,
//! speed u8, route u32 le, TT hub slot u8, TT port u8, EP0 size u16 le`
//! (12 bytes). The reply is a status: 0, or E_INVAL for a body this
//! refuses, E_BUSY with every virtual port in use.

use crate::table::{Parked, Virt, VPORT_BASE};

pub const PARK_LEN: usize = 12;

/// One 4-bit hub port per tier from the lowest, at most five.
fn route_ok(route: u32) -> bool {
    let tiers = (0..5).take_while(|t| (route >> (4 * t)) & 0xF != 0).count();
    tiers > 0 && route >> (4 * tiers as u32) == 0
}

/// A park body, checked for shape; the host checks the slots are its.
pub fn parse_park(body: &[u8]) -> Option<Parked> {
    let &[slot, parent, parent_port, speed, r0, r1, r2, r3, ts, tp, m0, m1] = body else {
        return None;
    };
    let route = u32::from_le_bytes([r0, r1, r2, r3]);
    let mps0 = u16::from_le_bytes([m0, m1]);
    let shaped = slot != 0 && parent != 0 && slot != parent && (1..=15).contains(&parent_port);
    let tt_ok = (ts == 0) == (tp == 0) && ts != slot;
    // Endpoint 0 sizes any device can have (USB 2.0 and 3.2 9.6.1).
    let mps0_ok = matches!(mps0, 8 | 16 | 32 | 64 | 512);
    let ok = shaped && (1..=5).contains(&speed) && route_ok(route) && tt_ok && mps0_ok;
    let (tt, connected, changed, claimant) = ((ts, tp), true, true, None);
    let p =
        Parked { slot, parent, parent_port, speed, route, tt, mps0, connected, changed, claimant };
    ok.then_some(p)
}

impl Virt {
    /// `p` listed: its virtual port, or None when its slot is known here
    /// already or every port is in use.
    pub fn park(&mut self, p: Parked) -> Option<u8> {
        if self.knows(p.slot) {
            return None;
        }
        let i = self.list.iter().position(|e| e.is_none())?;
        self.list[i] = Some(p);
        Some(VPORT_BASE + i as u8)
    }
}
