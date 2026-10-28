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
//! What the front needs of one controller.
//! OP_PARK from driver.usb_hub0 (nonos_usb_virt's park.rs has the body):
//! a child it addressed below a hub listed as a virtual port. The child
//! and its hub must be addressed on one controller here, and both slots
//! the sender's, as driver.xhci0 holds them (its slots/gate.rs).

use super::front::{Bind, Front};
use super::hc::Hc;
use crate::server::reply::status;
use crate::wire::{Request, E_BUSY, E_INVAL, E_PERM};
use nonos_usb_virt::parse_park;

pub fn park<H: Hc>(f: &mut Front<H>, pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    let Some(p) = parse_park(body) else { return status(tx, req, E_INVAL) };
    // The child's slot is the sender's (serve.rs); its hub's must be too.
    if !f.admits(p.parent, pid) {
        return status(tx, req, E_PERM);
    }
    let on = |s: u8| match f.bound.get((s as usize).wrapping_sub(1)) {
        Some(Bind::On(k)) => Some(*k),
        _ => None,
    };
    match (on(p.slot), on(p.parent)) {
        (Some(a), Some(b)) if a == b && !f.virt.knows(p.parent) => match f.virt.park(p) {
            Some(_) => status(tx, req, 0),
            None if f.virt.knows(p.slot) => status(tx, req, E_INVAL),
            None => status(tx, req, E_BUSY),
        },
        _ => status(tx, req, E_INVAL),
    }
}
