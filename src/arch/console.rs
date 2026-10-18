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

//! The debug console byte sink.
//!
//! Every architecture brings up one UART before anything else exists, because
//! a boot with no console cannot be diagnosed. x86_64 drives the PC 16550 at
//! I/O port 0x3F8; aarch64 drives the UART the device tree names for the board.
//! Only the byte-level access differs, so that is all this boundary carries:
//! the framing, locking and formatting stay in `sys::serial` above it.

/// Configure the console UART. Safe to call more than once.
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::console::init();
    #[cfg(target_arch = "aarch64")]
    uart::init();
}

/// True once the console can accept bytes.
pub fn is_available() -> bool {
    #[cfg(target_arch = "x86_64")]
    return crate::arch::x86_64::console::is_available();
    #[cfg(target_arch = "aarch64")]
    return uart::is_available();
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    return false;
}

/// Emit one byte, waiting a bounded time for the transmitter to have room.
pub fn write_byte(ch: u8) {
    #[cfg(target_arch = "x86_64")]
    crate::arch::x86_64::console::write_byte(ch);
    #[cfg(target_arch = "aarch64")]
    uart::write_byte(ch);
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    let _ = ch;
}

/// The aarch64 console is whichever UART the device tree named, PL011,
/// 16550 or mini-UART; the arch UART module records which and drives it.
#[cfg(target_arch = "aarch64")]
mod uart {
    use crate::arch::aarch64::uart;

    pub(super) fn init() {
        // Boot has already opened the console on the UART the device tree
        // names, at the speed it names, so there is nothing left to set.
    }

    pub(super) fn is_available() -> bool {
        uart::uart_base() != 0
    }

    pub(super) fn write_byte(ch: u8) {
        uart::write_byte(ch);
    }
}
