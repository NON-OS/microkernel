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

//! The console QEMU `virt` gives a guest that boots without a device tree.

use crate::arch::fdt::find::uart::{UartInfo, UartKind};

/// virt's PL011 runs from a fixed 24 MHz clock, so its divisor can be set
/// rather than inherited; QEMU accepts any, but the figure is the board's.
pub(super) const VIRT_UART: UartInfo = UartInfo {
    kind: UartKind::Pl011,
    base: 0x0900_0000,
    clock_hz: Some(24_000_000),
    baud: None,
    reg_shift: 0,
    io_width: 4,
};
