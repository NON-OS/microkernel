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

//! A bind's pending slots and GIC handlers, one per vector.

use crate::arch::aarch64::gic;
use crate::hardware::broker::irq::types::IrqBindError;

use super::super::pending_alloc::{claim, grant_ids};
use super::super::pending_find::{find_by_grant, free};
use super::super::trampoline;
use super::record::MessageBind;

/// A slot and a handler for every vector, all or none, under consecutive
/// grants. Returns the first.
pub(super) fn open(b: &MessageBind, epoch: u64) -> Result<u64, IrqBindError> {
    let vectors = &b.vectors[..b.count];
    let base = grant_ids(b.count as u64);
    for (i, v) in vectors.iter().enumerate() {
        if claim(v.intid, base + i as u64, b.pid, b.device_id, epoch, true).is_none() {
            (0..i).for_each(|n| close(base + n as u64, None));
            return Err(IrqBindError::AlreadyBound);
        }
    }
    for (i, v) in vectors.iter().enumerate() {
        if gic::register_irq_handler_for_capsule(v.intid, trampoline::handle).is_err() {
            vectors[..i].iter().for_each(|v| close(0, Some(v.intid)));
            (0..b.count).for_each(|n| close(base + n as u64, None));
            return Err(IrqBindError::PlatformError);
        }
    }
    Ok(base)
}

/// Free grant `grant`'s slot, and with `unhook` the handler on that INTID.
/// The handler goes only once the controller has dropped the vector: an
/// LPI with no handler is an unhandled interrupt, one with no slot is not.
pub(super) fn close(grant: u64, unhook: Option<u32>) {
    if let Some(intid) = unhook {
        let _ = gic::unregister_irq_handler_for_capsule(intid);
    }
    if let Some(e) = find_by_grant(grant) {
        free(e);
    }
}
