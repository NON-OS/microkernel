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
//! An address request: on a root port, sent to the controller that owns
//! it with the port renumbered both ways, the slot bound there if it is
//! addressed; on a virtual port, a claim on the parked child, answered
//! here as an address would be, with the claimant's own slot.

use super::front::{Bind, Front};
use super::hc::Hc;
use crate::server::reply::{bytes, status, DATA_AT};
use crate::wire::{Request, E_BUSY, E_INVAL, HDR_LEN};
use nonos_usb_virt::is_virtual;

pub fn address<H: Hc>(f: &mut Front<H>, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    let (Some(&slot), Some(&port)) = (body.first(), body.get(1)) else {
        return status(tx, req, E_INVAL);
    };
    let at = (slot as usize).wrapping_sub(1);
    if f.bound.get(at) != Some(&Bind::Enabled) || body.len() > 16 {
        return status(tx, req, E_INVAL);
    }
    if is_virtual(port) {
        if body.len() != 2 {
            return status(tx, req, E_INVAL);
        }
        let Some(p) = f.virt.claim(port, slot) else { return status(tx, req, E_BUSY) };
        let [m0, m1] = p.mps0.to_le_bytes();
        return bytes(tx, req, &[slot, port, p.speed, 0, m0, m1, 0, 0]);
    }
    let Some((k, local)) = f.local(port) else { return status(tx, req, E_INVAL) };
    let hc = f.hcs[k].as_mut().expect("a controller in place");
    let mut own = [0u8; 16];
    let own = &mut own[..body.len()];
    own.copy_from_slice(body);
    own[1] = local;
    hc.hold(slot);
    let n = hc.serve(req, own, tx);
    if i32::from_le_bytes([tx[HDR_LEN], tx[HDR_LEN + 1], tx[HDR_LEN + 2], tx[HDR_LEN + 3]]) == 0 {
        (f.bound[at], tx[DATA_AT + 1]) = (Bind::On(k), port);
    } else {
        hc.release(slot);
    }
    n
}
