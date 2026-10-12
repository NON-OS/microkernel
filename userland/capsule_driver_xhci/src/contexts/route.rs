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
//! The Slot Context fields that place a device below a hub and mark a hub
//! (xHCI 1.2 section 6.2.2, as recalled, unverified): the route string in
//! dword 0 bits 19:0, MTT bit 25 and Hub bit 26; Number of Ports in dword
//! 1 bits 31:24; the TT's hub slot (bits 7:0), port (15:8) and think time
//! (17:16) in dword 2. Pure, for xhci_proofs.

const ROUTE_MASK: u32 = 0x000F_FFFF;
const MTT: u32 = 1 << 25;
const HUB: u32 = 1 << 26;
const PORTS_SHIFT: u32 = 24;
const TTT_SHIFT: u32 = 16;

/// Whether `route` names a place below a root port: one 4-bit hub port
/// per tier from the lowest, none after the first empty one, at most five.
pub fn route_ok(route: u32) -> bool {
    if route == 0 || route & !ROUTE_MASK != 0 {
        return false;
    }
    let tiers = (0..5).take_while(|t| (route >> (4 * t)) & 0xF != 0).count();
    route >> (4 * tiers as u32) == 0
}

/// Whether a device at `speed` (xHCI speed ids 1 to 5) may be addressed
/// at `route` through `tt`: a TT names a hub's port 1 to 15
/// (`tt_is_hub`) and serves only full (1) or low (2) speed.
pub fn routed_fits(speed: u8, route: u32, tt: Option<(u8, u8)>, tt_is_hub: bool) -> bool {
    let tt_fits =
        tt.is_none_or(|(_, port)| tt_is_hub && (1..=15).contains(&port) && matches!(speed, 1 | 2));
    (1..=5).contains(&speed) && route_ok(route) && tt_fits
}

/// Slot dword 0 with `route` in place of whatever route it held.
pub fn with_route(dw0: u32, route: u32) -> u32 {
    (dw0 & !ROUTE_MASK) | (route & ROUTE_MASK)
}

/// Slot dword 2 naming the TT a low or full speed device's split
/// transactions go through; the interrupter target and think time kept.
pub fn with_tt(dw2: u32, hub_slot: u8, port: u8) -> u32 {
    (dw2 & !0xFFFF) | hub_slot as u32 | (port as u32) << 8
}

/// Slot dwords 0, 1 and 2 of a hub: Hub set, MTT as asked, its port count
/// and its TT think time (0 to 3, in 8 FS bit times each).
pub fn as_hub(dws: [u32; 3], ports: u8, think: u8, mtt: bool) -> [u32; 3] {
    let dw0 = (dws[0] & !MTT) | HUB | if mtt { MTT } else { 0 };
    let dw1 = (dws[1] & !(0xFF << PORTS_SHIFT)) | (ports as u32) << PORTS_SHIFT;
    let dw2 = (dws[2] & !(0x3 << TTT_SHIFT)) | ((think & 0x3) as u32) << TTT_SHIFT;
    [dw0, dw1, dw2]
}
