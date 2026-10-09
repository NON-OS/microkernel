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

//! Choosing how a unit coming into service is invalidated, and saying so.

use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::arch::x86_64::iommu::unit::queue;
use crate::sys::serial::Line;

/// Start the queue where the unit has one. A queue that will not start leaves
/// the unit on its registers, as Linux falls back in intel/iommu.c, init_dmars;
/// interrupt remapping then stays off on that unit, since it needs the queue.
///
/// # Safety
/// Bring-up only, before the unit translates with this kernel's tables.
pub(super) unsafe fn start_invalidation(unit: &RemapUnit) {
    let mut line = Line::new();
    line.str(b"[VT-D] IOMMU unit base=").hex(unit.base_pa());
    // SAFETY: eK@nonos.systems - the caller's promise; nothing else submits
    // to a unit that is not yet in service.
    line.str(match unsafe { queue::enable(unit) } {
        Ok(true) => b" invalidation=queue",
        Ok(false) => b" invalidation=registers (no QI)",
        Err(_) => b" invalidation=registers (queue did not start)",
    });
    line.end();
}
