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
//! 16-bit port space, its count converted and never masked, and clear of the
//! machine's own ports (`window`, K6) and of the FADT's power registers
//! (`power_ports`). A window past 0xFFFF would wrap onto port 0 on the wire,
//! so it is refused rather than truncated.

use super::power_ports::clear;
use super::types::PioError;
use super::window::{window, Refused};
use crate::hardware::broker::device::BAR_KIND_PIO;
use crate::hardware::broker::Bar;

pub(super) fn port_window(bar: Bar) -> Result<(u16, u16), PioError> {
    if bar.kind != BAR_KIND_PIO {
        return Err(PioError::NotPioBar);
    }
    if bar.size == 0 {
        return Err(PioError::ZeroSize);
    }
    clear(window(bar.base, bar.size)).map_err(|r| match r {
        Refused::Overflow => PioError::PortOverflow,
        Refused::Reserved => PioError::ReservedPort,
    })
}
