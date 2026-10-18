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
//! Slots given out at the host, and a request carried to the controller
//! its slot is bound to; a claimant's slot stands for the parked one
//! (nonos_usb_virt). give_back.rs gives slots back.

use super::front::{Bind, Front};
use super::hc::Hc;
use crate::server::reply::{bytes, status};
use crate::wire::{Request, E_BUSY, E_INVAL, REQUEST_MAX};

pub fn enable<H: Hc>(f: &mut Front<H>, req: &Request, tx: &mut [u8]) -> usize {
    match f.bound.iter().position(|b| *b == Bind::Free) {
        Some(i) => {
            f.bound[i] = Bind::Enabled;
            bytes(tx, req, &[i as u8 + 1, 0, 0, 0])
        }
        None => status(tx, req, E_BUSY),
    }
}

/// `slot`'s request on its controller, as `body` with `slot` first; the
/// slot is free again once a disable is served.
pub(super) fn on<H: Hc>(
    f: &mut Front<H>,
    slot: u8,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let at = (slot as usize).wrapping_sub(1);
    let Some(Bind::On(k)) = f.bound.get(at).copied() else { return status(tx, req, E_INVAL) };
    let hc = f.hcs[k].as_mut().expect("a controller in place");
    let n = if body.first() == Some(&slot) {
        hc.serve(req, body, tx)
    } else {
        let mut own = [0u8; REQUEST_MAX];
        let len = body.len().min(REQUEST_MAX);
        own[..len].copy_from_slice(&body[..len]);
        own[0] = slot;
        hc.serve(req, &own[..len], tx)
    };
    if req.op == crate::wire::ops::OP_DISABLE_SLOT {
        f.bound[at] = Bind::Free;
    }
    n
}

pub fn forward<H: Hc>(f: &mut Front<H>, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    let Some(&slot) = body.first() else { return status(tx, req, E_INVAL) };
    let real = f.virt.real(slot);
    on(f, real, req, body, tx)
}
