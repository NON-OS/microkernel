// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The PC's I/O ports: COM1 for a byte after Boot Services, port 0x80 for
//! a POST code, and the 0xCF8/0xCFC window into PCI configuration space.

/// One byte to COM1.
#[inline(always)]
pub fn post_byte(b: u8) {
    // SAFETY: COM1's data port; a write changes nothing but the UART.
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3F8u16,
            in("al") b,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// The POST code to port 0x80, then halted.
pub fn halt_with_code(code: u8) -> ! {
    // SAFETY: port 0x80 is the standard POST debug port, write-only
    unsafe {
        core::arch::asm!("out 0x80, al", in("al") code, options(nomem, nostack));
    }
    loop {
        core::hint::spin_loop();
    }
}

/// A dword of PCI configuration space through the legacy window.
pub fn pci_config_read(addr: u32) -> Option<u32> {
    let data: u32;
    // SAFETY: 0xCF8 selects and 0xCFC reads; neither touches memory.
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") 0xCF8u16,
            in("eax") addr & 0xFFFF_FFFC,
            options(nomem, nostack, preserves_flags)
        );
        core::arch::asm!(
            "in eax, dx",
            out("eax") data,
            in("dx") 0xCFCu16,
            options(nomem, nostack, preserves_flags)
        );
    }
    Some(data)
}
