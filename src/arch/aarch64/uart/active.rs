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

use crate::arch::fdt::find::uart::UartKind;

use super::line::Line;
use super::ns16550::Ns16550;
use super::pl011::Pl011;
use super::regs::Regs;
use super::state;

/// The console as the state last recorded it.
pub(super) enum Active {
    Pl011(Pl011),
    Ns16550(Ns16550),
    MiniUart(Ns16550),
}

impl Active {
    /// `None` when no console has an address.
    pub(crate) fn current() -> Option<Self> {
        let base = state::uart_base();
        if base == 0 {
            return None;
        }
        let (kind, shift, width) = state::layout();
        let regs = Regs { base, shift, width };
        Some(match kind {
            UartKind::Pl011 => Self::Pl011(Pl011::new(base)),
            UartKind::Ns16550 => Self::Ns16550(Ns16550::new(regs)),
            UartKind::MiniUart => Self::MiniUart(Ns16550::new(regs)),
        })
    }

    pub(crate) fn init(&self, line: Line) {
        match self {
            Self::Pl011(u) => u.init(line),
            Self::Ns16550(u) => u.init(line),
            Self::MiniUart(u) => u.init_mini(line),
        }
    }

    /// Reading the data drops a 16550's receive interrupt by itself; the
    /// PL011 latches its causes until they are cleared.
    pub(crate) fn acknowledge(&self) {
        if let Self::Pl011(u) = self {
            u.clear_interrupts();
        }
    }
}
