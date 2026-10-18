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

use super::kind::UartKind;

/// `(reg_shift, io_width)`, or `None` for a layout we cannot drive. The
/// mini-UART's is fixed by the chip; a 16550 states its own, defaulting to
/// the original byte-wide, byte-spaced part.
pub(super) fn layout(
    fdt: &Fdt,
    node: &NodeRef,
    kind: UartKind,
) -> Result<Option<(u8, u8)>, FdtError> {
    match kind {
        UartKind::Pl011 => Ok(Some((0, 4))),
        UartKind::MiniUart => Ok(Some((2, 4))),
        UartKind::Ns16550 => {
            let shift = cell(fdt, node, b"reg-shift", 0)?;
            let width = cell(fdt, node, b"reg-io-width", 1)?;
            match (shift, width) {
                (Some(s @ 0..=2), Some(w @ (1 | 4))) => Ok(Some((s as u8, w as u8))),
                _ => Ok(None),
            }
        }
    }
}

/// A one-cell property, `default` when absent, `None` when malformed.
fn cell(fdt: &Fdt, node: &NodeRef, name: &[u8], default: u32) -> Result<Option<u32>, FdtError> {
    match fdt.property(node, name)? {
        None => Ok(Some(default)),
        Some(p) if p.data.len() == 4 => Ok(Some(p.u32()?)),
        Some(_) => Ok(None),
    }
}
