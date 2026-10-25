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

//! The MADT entries the madt tests feed to `plan`, and the run itself.

use super::madt_plan::{plan, MadtCpu, PlanCounts};

pub(super) fn lapic(id: u32) -> MadtCpu {
    MadtCpu { apic_id: id, enabled: true, x2apic_entry: false }
}
pub(super) fn x2(id: u32) -> MadtCpu {
    MadtCpu { apic_id: id, enabled: true, x2apic_entry: true }
}
pub(super) fn run(
    entries: &[MadtCpu],
    own: u32,
    x2mode: bool,
    max: usize,
) -> (Vec<u32>, PlanCounts) {
    let mut out = Vec::new();
    let c = plan(entries, own, x2mode, max, |id| out.push(id));
    (out, c)
}
