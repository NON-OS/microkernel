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

//! A line bind kept only while its claim stands (L77), as x86's records are
//! entered (L69). Asked after the enable: a drain finds the slot from its
//! allocation on, so a claim standing here is drained after. Refused, the
//! bind is undone by its grant id, which no other bind carries; if a drain
//! freed the slot first, the line is let go only while no slot names it.

use crate::arch::aarch64::gic;
use crate::arch::aarch64::gic::irq_handlers;
use crate::hardware::broker::claim;
use crate::hardware::broker::irq::types::IrqBindError;

pub(super) fn keep(
    pid: u32,
    device_id: u64,
    epoch: u64,
    intid: u32,
    grant_id: u64,
) -> Result<(), IrqBindError> {
    if claim::live(pid, device_id, epoch) {
        return Ok(());
    }
    if super::release::unmap_grant(pid, grant_id).is_err()
        && super::pending_find::find_by_intid(intid).is_none()
    {
        gic::disable_irq(intid);
        let _ = irq_handlers::unregister_for_capsule(intid);
    }
    Err(IrqBindError::NotClaimed)
}
