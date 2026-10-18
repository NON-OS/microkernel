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

//! The console UART. Boards differ in which one they wire up: QEMU's virt
//! and most servers give a PL011, the Pi's default console is the BCM2835
//! mini-UART, and SoC boards often have a 16550 at some stride. The device
//! tree says which; this module drives whichever it said.

mod active;
mod api;
mod divisor;
mod interrupt;
mod line;
mod ns16550;
mod pl011;
mod regs;
mod state;
mod transfer;
mod wait;

pub use api::{getc, init_uart, putc, puts, write_byte};
pub use interrupt::handle_uart_interrupt;
pub use state::uart_base;
