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
use crate::constants::{PORTSC_BASE, PORT_REG_STRIDE};
use crate::error::{XhciError, XhciResult};
const WINDOW_GUARD: u64 = 0x1000;
pub fn require_window(offset: u64, mapped_len: u64) -> XhciResult<()> {
    let end = offset.checked_add(WINDOW_GUARD).ok_or(XhciError::ControllerUnsupported)?;
    if end > mapped_len {
        return Err(XhciError::ControllerUnsupported);
    }
    Ok(())
}

/// Every root port's register set, 16 bytes each from operational offset
/// 0x400 (xHCI 1.2 section 5.4.8), lies inside the mapped window. A device
/// claiming the xHCI class with a small BAR and up to 255 ports, as a
/// hostile PCIe or Thunderbolt one can, is refused here; read past the
/// mapping at every port access, it page-faulted the capsule.
pub fn require_ports(cap_len: u64, max_ports: u8, mapped_len: u64) -> XhciResult<()> {
    let sets = max_ports as u64 * PORT_REG_STRIDE;
    match cap_len.checked_add(PORTSC_BASE + sets) {
        Some(end) if end <= mapped_len => Ok(()),
        _ => Err(XhciError::ControllerUnsupported),
    }
}
