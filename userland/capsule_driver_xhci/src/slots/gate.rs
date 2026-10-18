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

//! Every request that names a slot, held to who may make it. A slot no
//! interface of is claimed serves only the driver that enabled it, so its
//! first claim is that driver's too. A shared one serves its claimants,
//! each pipe only to the one that configured it (owners.rs); anyone may
//! read its descriptors and claim a free interface, which a driver joining
//! it must do, and its other slot requests are its claimants' or holder's.

use super::asker::Asker;
use super::owners::E_PERM;
use super::sharing::Sharing;
use super::slot_ops::names_slot;
use super::uses::use_of;
use crate::protocol::*;

pub fn gate(
    s: &mut Sharing,
    op: u16,
    body: &[u8],
    pid: u32,
    alive: impl Fn(u32) -> bool,
) -> Result<(), i32> {
    let Some(&slot) = body.first().filter(|_| names_slot(op)) else { return Ok(()) };
    let shared = s.claims.any(slot);
    let holds = s.claims.holds(slot, pid);
    let refuse = |ok: bool| if ok { Ok(()) } else { Err(E_PERM) };
    match use_of(op, body) {
        _ if !shared && !s.holders.admits(slot, pid, &alive) => Err(E_PERM),
        // Unshared too, so a pipe configured before the device is shared
        // stays its configurer's once it is.
        Some((slot, u)) => s.owners.admit(slot, u, Asker { pid, shared, holds }),
        None if !shared => Ok(()),
        None if matches!(op, OP_GET_DEVICE_DESCRIPTOR | OP_GET_CONFIG_DESCRIPTOR) => Ok(()),
        None if op == OP_CLAIM_INTERFACE => Ok(()),
        None => refuse(holds || s.holders.admits(slot, pid, &alive)),
    }
}

/// Parking a hub's child (OP_PARK) takes it out from under whoever drives
/// it, so only a driver admitted on both the child's slot and its hub's
/// parks it: a claimant, or the holder while none is; anyone else gets -1.
pub fn gate_park(
    s: &mut Sharing,
    child: u8,
    hub: u8,
    pid: u32,
    alive: impl Fn(u32) -> bool,
) -> Result<(), i32> {
    let mut on = |slot| match s.claims.any(slot) {
        true => s.claims.holds(slot, pid),
        false => s.holders.admits(slot, pid, &alive),
    };
    if on(hub) && on(child) {
        Ok(())
    } else {
        Err(E_PERM)
    }
}
