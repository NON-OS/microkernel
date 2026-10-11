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

//! Global errors: the unit's own faults, such as a refused command or an
//! MSI that could not be written. Each is reported and acknowledged; an
//! active one left alone would hide the next.

use crate::sys::serial::Line;

use super::mmio::Regs;
use super::regs::{GERROR, GERRORN};

pub(super) fn report(regs: Regs) {
    let raised = regs.read32(GERROR);
    let active = raised ^ regs.read32(GERRORN);
    if active == 0 {
        return;
    }
    Line::new().str(b"[SMMU] global error ").hex(u64::from(active)).end();
    // Acknowledged by making GERRORN match GERROR for those bits.
    regs.write32(GERRORN, raised);
}
