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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;
use crate::arch::fdt::property::Property;

/// The UART's input clock: its own `clock-frequency`, or the rate of the
/// fixed clock its first `clocks` entry names. A clock behind a controller
/// (the Pi's, which firmware sets) has no rate the tree can give, so it is
/// `None` rather than a guess that would garble every byte.
pub(super) fn clock_hz(fdt: &Fdt, node: &NodeRef) -> Result<Option<u32>, FdtError> {
    if let Some(prop) = fdt.property(node, b"clock-frequency")? {
        return Ok(rate(&prop));
    }
    let Some(clocks) = fdt.property(node, b"clocks")? else {
        return Ok(None);
    };
    let Ok(phandle) = clocks.u32() else {
        return Ok(None);
    };
    let clock = fdt.by_phandle(phandle)?;
    if fdt.compatible_with(&clock, &[b"fixed-clock"])?.is_none() {
        return Ok(None);
    }
    Ok(fdt.property(&clock, b"clock-frequency")?.as_ref().and_then(rate))
}

/// One cell, or two that fit in one; the binding allows either.
fn rate(prop: &Property) -> Option<u32> {
    let hz = match *prop.data {
        [a, b, c, d] => u32::from_be_bytes([a, b, c, d]) as u64,
        [a, b, c, d, e, f, g, h] => u64::from_be_bytes([a, b, c, d, e, f, g, h]),
        _ => return None,
    };
    u32::try_from(hz).ok().filter(|&hz| hz != 0)
}
