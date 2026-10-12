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

//! The interpreter's \_S5, put where the kernel's soft-off reads it. The
//! constant reader at boot finds \_S5 only when firmware wrote it as a
//! plain package; a laptop that computes it in a method left soft-off
//! unavailable, and the machine parked instead of powering off. Shutdown
//! itself is unchanged: it still reads the registry, never AML.

use super::kernel::platform::fw::aml::sleep_obj::SleepPackage;
use super::kernel::platform::fw::parser::state::TABLES;

/// Returns what the registry held before; None when the ACPI tables were
/// never read, and there is nowhere to put it.
pub fn adopt_s5(a: u8, b: u8) -> Option<Option<(u8, u8)>> {
    let mut tables = TABLES.write();
    let reg = tables.as_mut()?;
    let before = reg.data.s5.map(|p| (p.slp_typ_a, p.slp_typ_b));
    reg.data.s5 = Some(SleepPackage { slp_typ_a: a, slp_typ_b: b });
    Some(before)
}
