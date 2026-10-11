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

use crate::arch::fdt::find::uart::{UartInfo, UartKind};

use super::divisor;

/// The speed every board here boots its console at when nothing says
/// otherwise, and the one the Pi firmware and QEMU both assume.
const DEFAULT_BAUD: u32 = 115_200;

/// What `init_uart` does to the line settings.
#[derive(Debug, Clone, Copy)]
pub(super) enum Line {
    /// No clock rate is known, so firmware's divisor is the only one that
    /// works: enable the port and leave its speed alone.
    Keep,
    Pl011 {
        integer: u32,
        fraction: u32,
    },
    Ns16550(u16),
    MiniUart(u16),
}

/// Worked out before any register is touched, so an impossible rate is
/// refused while the previous console still works to say so.
pub(super) fn line(info: &UartInfo) -> Result<Line, &'static str> {
    let Some(clock) = info.clock_hz else {
        return Ok(Line::Keep);
    };
    let baud = info.baud.unwrap_or(DEFAULT_BAUD);
    let line = match info.kind {
        UartKind::Pl011 => {
            divisor::pl011(clock, baud).map(|(integer, fraction)| Line::Pl011 { integer, fraction })
        }
        UartKind::Ns16550 => divisor::ns16550(clock, baud).map(Line::Ns16550),
        UartKind::MiniUart => divisor::mini_uart(clock, baud).map(Line::MiniUart),
    };
    line.ok_or("the console's clock cannot make its baud rate")
}
