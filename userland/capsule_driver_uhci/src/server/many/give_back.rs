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
//! A slot given back. For a parked child or its claimant it means what
//! the virtual port table says (nonos_usb_virt's disable.rs): a detach,
//! a claim let go, or both slots gone.

use super::front::{Bind, Front};
use super::hc::Hc;
use super::slots::on;
use crate::server::reply::status;
use crate::wire::{Request, E_INVAL};
use nonos_usb_virt::Disable;

pub fn disable<H: Hc>(f: &mut Front<H>, req: &Request, body: &[u8], tx: &mut [u8]) -> usize {
    let &[slot] = body else { return status(tx, req, E_INVAL) };
    let at = (slot as usize).wrapping_sub(1);
    let free = |f: &mut Front<H>| f.bound[at] = Bind::Free;
    match f.virt.disable(slot) {
        Disable::Detached => status(tx, req, 0),
        Disable::Release => {
            free(f);
            status(tx, req, 0)
        }
        Disable::ReleaseGone(parked) => {
            free(f);
            on(f, parked, req, &[parked], tx)
        }
        Disable::Detach(_) | Disable::Plain => match f.bound.get(at).copied() {
            Some(Bind::Enabled) => {
                free(f);
                status(tx, req, 0)
            }
            Some(Bind::On(_)) => on(f, slot, req, body, tx),
            _ => status(tx, req, E_INVAL),
        },
    }
}
