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

use crate::arch::fdt::parser::Fdt;

use super::read::state;
use super::refusal::Refusal;
use super::types::{IdleState, MAX_STATES};

/// The states under `/cpus/idle-states`, in tree order. None unless its
/// `entry-method` is PSCI, the only way in the kernel has; the older
/// "arm,psci" spelling is taken too (both recalled).
pub fn states(fdt: &Fdt, out: &mut [IdleState; MAX_STATES]) -> Result<usize, Refusal> {
    let Some(parent) = fdt.find_path(b"/cpus/idle-states")? else {
        return Ok(0);
    };
    let psci = match fdt.property(&parent, b"entry-method")? {
        Some(method) => matches!(method.string()?, b"psci" | b"arm,psci"),
        None => false,
    };
    if !psci {
        return Ok(0);
    }
    let mut filled = 0usize;
    for node in fdt.children(&parent)? {
        let Some(found) = state(fdt, &node?)? else {
            continue;
        };
        *out.get_mut(filled).ok_or(Refusal::TooManyStates)? = found;
        filled += 1;
    }
    Ok(filled)
}
