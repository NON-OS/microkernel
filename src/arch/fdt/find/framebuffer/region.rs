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

//! Where the framebuffer's memory is, in CPU terms.
//!
//! Firmware often puts the node under `/chosen` (the binding suggests it),
//! and `/chosen` is not a bus: it often carries no `ranges`, and its `reg`
//! values are then the CPU's own addresses. A node anywhere else is
//! translated through its buses' `ranges` like any device.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

pub(super) fn region(fdt: &Fdt, node: &NodeRef) -> Result<Option<(u64, u64)>, FdtError> {
    match fdt.mmio(node, 0) {
        Err(FdtError::NotTranslatable) if under_chosen(fdt, node)? => {}
        other => return other,
    }
    let Some(mut reg) = fdt.reg(node)? else {
        return Ok(None);
    };
    if !reg.readable() {
        return Err(FdtError::BadCells);
    }
    let Some((base, size)) = reg.next() else {
        return Ok(None);
    };
    base.checked_add(size).ok_or(FdtError::OutOfBounds)?;
    Ok(Some((base, size)))
}

fn under_chosen(fdt: &Fdt, node: &NodeRef) -> Result<bool, FdtError> {
    if node.depth != 2 {
        return Ok(false);
    }
    Ok(fdt.chain(node)?.as_slice().get(1).is_some_and(|parent| parent.name == b"chosen"))
}
