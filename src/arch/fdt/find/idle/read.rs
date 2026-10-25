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

//! One child of `/cpus/idle-states` (the Arm idle-state binding, recalled).

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

use super::types::IdleState;

/// The state a node describes, or `None` when it is not one the kernel may
/// enter: not `arm,idle-state`, disabled, unnamed by any phandle, or missing
/// its power_state or a latency. None of those are guessed.
pub(super) fn state(fdt: &Fdt, node: &NodeRef) -> Result<Option<IdleState>, FdtError> {
    let idle = match fdt.property(node, b"compatible")? {
        Some(c) => c.compatible_matches(b"arm,idle-state"),
        None => false,
    };
    if !idle || !fdt.is_enabled(node)? {
        return Ok(None);
    }
    let phandle = match word(fdt, node, b"phandle")? {
        Some(p) => Some(p),
        None => word(fdt, node, b"linux,phandle")?,
    };
    let fields = (
        phandle,
        word(fdt, node, b"arm,psci-suspend-param")?,
        word(fdt, node, b"entry-latency-us")?,
        word(fdt, node, b"exit-latency-us")?,
        word(fdt, node, b"min-residency-us")?,
    );
    let (Some(phandle), Some(param), Some(entry_us), Some(exit_us), Some(residency_us)) = fields
    else {
        return Ok(None);
    };
    Ok(Some(IdleState {
        phandle,
        param,
        entry_us,
        exit_us,
        residency_us,
        timer_stops: fdt.property(node, b"local-timer-stop")?.is_some(),
    }))
}

/// A one-cell property. Present at any other length, it is damaged, and a
/// damaged state is refused rather than read short.
fn word(fdt: &Fdt, node: &NodeRef, name: &[u8]) -> Result<Option<u32>, FdtError> {
    match fdt.property(node, name)? {
        Some(p) if p.data.len() == 4 => Ok(Some(p.u32()?)),
        Some(_) => Err(FdtError::BadCells),
        None => Ok(None),
    }
}
