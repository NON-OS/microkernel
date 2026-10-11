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

use super::clock::clock_hz;
use super::compatible::kind;
use super::info::UartInfo;
use super::layout::layout;
use super::speed::baud;

/// Everything a driver needs from one matched node, or `None` when it has
/// no MMIO window or a register layout we cannot drive.
pub(super) fn describe(
    fdt: &Fdt,
    node: &NodeRef,
    needle: usize,
    options: &[u8],
) -> Result<Option<UartInfo>, FdtError> {
    let kind = kind(needle);
    let Some((base, _)) = fdt.mmio(node, 0)? else {
        return Ok(None);
    };
    let Some((reg_shift, io_width)) = layout(fdt, node, kind)? else {
        return Ok(None);
    };
    Ok(Some(UartInfo {
        kind,
        base,
        clock_hz: clock_hz(fdt, node)?,
        baud: baud(fdt, node, options)?,
        reg_shift,
        io_width,
    }))
}
