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

//! The port window a BAR names: an I/O BAR with a size, wholly inside the
//! 16-bit port space. A window past 0xFFFF would wrap onto port 0 on the
//! wire, so it is refused rather than truncated.

use super::types::PioError;
use crate::hardware::broker::device::BAR_KIND_PIO;
use crate::hardware::broker::Bar;

pub(super) fn port_window(bar: Bar) -> Result<(u16, u16), PioError> {
    if bar.kind != BAR_KIND_PIO {
        return Err(PioError::NotPioBar);
    }
    if bar.size == 0 {
        return Err(PioError::ZeroSize);
    }
    if bar.base > u16::MAX as u64 {
        return Err(PioError::PortOverflow);
    }
    let port_base = bar.base as u16;
    let last_port = bar.base.checked_add(bar.size).ok_or(PioError::PortOverflow)?;
    if last_port > u16::MAX as u64 + 1 {
        return Err(PioError::PortOverflow);
    }
    let port_count = (bar.size & u16::MAX as u64) as u16;
    Ok((port_base, port_count))
}
