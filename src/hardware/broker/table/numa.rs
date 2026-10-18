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

//! Writing the records' `numa_node`, the one field the NUMA facts change.

use super::state::TABLE;

#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
pub(in crate::hardware::broker) fn set_numa_nodes(nodes: &[(u64, u8)]) {
    let mut table = TABLE.write();
    for r in table.iter_mut() {
        if let Some((_, n)) = nodes.iter().find(|(id, _)| *id == r.device_id) {
            r.numa_node = *n;
        }
    }
}
