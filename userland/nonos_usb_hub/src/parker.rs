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
//! The classes driver.usb_hub0 brings: a hub below a hub is configured and
//! brought up for the walk, and any other device is parked with the host
//! for the class drivers to claim (nonos_usb_virt). Nothing is bound here.

use alloc::vec::Vec;

use crate::attach::attach;
use crate::classes::Classes;
use crate::host::{Host, Setup};
use crate::node::Hub;
use crate::place::Place;
use crate::status_endpoint::status_endpoint;

/// A configuration read whole, its head first (USB 2.0 section 9.4.3).
const CONFIG_MAX: usize = 255;

pub struct Parker;

fn get<H: Host>(h: &mut H, slot: u8, kind: u16, buf: &mut [u8]) -> Option<usize> {
    let length = buf.len() as u16;
    let s = Setup { request_type: 0x80, request: 6, value: kind << 8, index: 0, length };
    h.control(slot, s, buf).ok()
}

/// A hub's configuration selected and the hub brought up: Some(hub).
pub fn as_hub<H: Host>(h: &mut H, slot: u8, place: Place) -> Option<Hub> {
    let mut raw = [0u8; CONFIG_MAX];
    let n = get(h, slot, 2, &mut raw[..9])?;
    let total = (u16::from_le_bytes([raw[2], raw[3]]) as usize).clamp(n, CONFIG_MAX);
    let n = get(h, slot, 2, &mut raw[..total])?;
    status_endpoint(&raw[..n])?;
    let set = Setup { request_type: 0, request: 9, value: raw[5] as u16, index: 0, length: 0 };
    h.control(slot, set, &mut []).ok()?;
    attach(h, slot, place, &raw[..n]).ok()
}

impl<H: Host> Classes<H> for Parker {
    fn classify(&mut self, h: &mut H, slot: u8, place: Place, hubs: &mut Vec<Hub>) -> bool {
        if let Some(hub) = as_hub(h, slot, place) {
            hubs.push(hub);
            return true;
        }
        let mut dev = [0u8; 8];
        let Some(8) = get(h, slot, 1, &mut dev) else { return false };
        h.park(slot, &place, dev[7].max(8) as u16).is_ok()
    }

    /// The host holds the parked child; giving back its slot tells it.
    fn forget(&mut self, _slot: u8) {}
}
