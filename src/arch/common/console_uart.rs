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

//! The console UARTs the device tree names, on aarch64 and riscv64.

/// The aarch64 console is whichever UART the device tree named, PL011,
/// 16550 or mini-UART; the arch UART module records which and drives it.
#[cfg(target_arch = "aarch64")]
pub(in crate::arch) mod uart {
    use crate::arch::aarch64::uart;

    pub(in crate::arch) fn init() {
        // Boot has already opened the console on the UART the device tree
        // names, at the speed it names, so there is nothing left to set.
    }

    pub(in crate::arch) fn is_available() -> bool {
        uart::uart_base() != 0
    }

    pub(in crate::arch) fn write_byte(ch: u8) {
        uart::write_byte(ch);
    }
}

/// The riscv64 console is the NS16550 the device tree names, opened by the
/// boot path before anything prints.
#[cfg(target_arch = "riscv64")]
pub(in crate::arch) mod rv_uart {
    use crate::arch::riscv64::uart;

    pub(in crate::arch) fn init() {}

    pub(in crate::arch) fn is_available() -> bool {
        uart::uart_base() != 0
    }

    /// Longer than a 16550 at 115200 baud takes to drain its FIFO, and
    /// finite, so a UART that never reports room cannot hold the hart.
    const TX_TRIES: u32 = 100_000;

    pub(in crate::arch) fn write_byte(ch: u8) {
        let port = uart::Ns16550::new(uart::uart_base());
        if (0..TX_TRIES).any(|_| port.is_tx_ready()) {
            port.putc(ch);
        }
    }
}
