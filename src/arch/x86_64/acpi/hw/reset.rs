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

//! Resetting a PC, in the order Linux's `native_machine_emergency_restart`
//! walks with its default `reboot=acpi`:
//!
//! 1. ACPI reset register (`drivers/acpi/reboot.c`), then 15 ms;
//! 2. keyboard controller pulse 0xFE to port 0x64, ten times, each after
//!    waiting for the input buffer to drain;
//! 3. ACPI reset register again, then the keyboard controller again;
//! 4. port 0xCF9 (`BOOT_CF9_FORCE`): 0x02 then the reset code, 50 us apart;
//! 5. the caller then triple faults.
//!
//! The ACPI reset register is used only when the FADT sets RESET_REG_SUP and
//! gives an address. A System I/O register is written with one byte whatever
//! its GAS width says, as Linux's `acpi_reset` does (Windows does the same and
//! firmware is tested against that); a System Memory register goes through
//! the GAS rules; a PCI configuration register is a byte write to bus 0 with
//! the device in address bits 32..47, the function in bits 16..31 and the
//! offset in bits 0..15 (ACPI 6.5 section 5.2.3.2).
//!
//! The 0xCF9 code is 0x0E, a cold reset with FULL_RST, which is what Linux
//! writes for its default (cold) reboot mode; 0x06 would be a warm reset that
//! keeps DRAM powered, which the zerostate path does not want.

use super::fadt_decode::FadtInfo;
use super::gas::{gas_write, RegisterBus, SPACE_PCI_CONFIG, SPACE_SYSTEM_IO, SPACE_SYSTEM_MEMORY};

pub const KBD_STATUS_PORT: u16 = 0x64;
pub const KBD_STATUS_IBF: u8 = 0x02;
pub const KBD_CMD_PULSE_RESET: u8 = 0xFE;
pub const RST_CNT_PORT: u16 = 0xCF9;
pub const RST_CNT_SYS_RST: u8 = 0x02;
pub const RST_CNT_COLD: u8 = 0x0E;

/// What resetting needs beyond plain register access.
pub trait ResetBus: RegisterBus {
    /// Byte write to PCI configuration space, segment 0 bus 0.
    fn pci_write8(&mut self, device: u8, function: u8, offset: u16, value: u8) -> bool;
    /// Busy wait. Must not depend on interrupts.
    fn delay_us(&mut self, us: u32);
}

/// The PCI location a PCI-configuration-space reset GAS names, or None when
/// it is out of range.
pub fn pci_reset_target(address: u64) -> Option<(u8, u8, u16)> {
    let device = (address >> 32) & 0xFFFF;
    let function = (address >> 16) & 0xFFFF;
    let offset = address & 0xFFFF;
    if device > 31 || function > 7 || offset > 0xFFF {
        return None;
    }
    Some((device as u8, function as u8, offset as u16))
}

/// Write the FADT reset register. True when a write was issued (the machine
/// may still not have reset). Waits 15 ms afterwards, as Linux does, so the
/// next method does not race a reset already in flight.
pub fn acpi_reset<B: ResetBus>(bus: &mut B, fadt: &FadtInfo) -> bool {
    if !fadt.has_reset_reg() {
        return false;
    }
    let reg = fadt.reset_reg;
    let issued = match reg.space {
        SPACE_SYSTEM_IO => {
            if reg.address <= 0xFFFF {
                bus.write_io(reg.address as u16, 8, fadt.reset_value as u32);
                true
            } else {
                false
            }
        }
        SPACE_SYSTEM_MEMORY => gas_write(bus, &reg, fadt.reset_value as u64),
        SPACE_PCI_CONFIG => match pci_reset_target(reg.address) {
            Some((dev, func, offset)) => bus.pci_write8(dev, func, offset, fadt.reset_value),
            None => false,
        },
        _ => false,
    };
    if issued {
        bus.delay_us(15_000);
    }
    issued
}

/// Linux `kb_wait`: up to 0x10000 polls, 2 us apart, for the 8042 input
/// buffer to empty.
fn kbd_wait<B: ResetBus>(bus: &mut B) {
    for _ in 0..0x10000u32 {
        if bus.read_io(KBD_STATUS_PORT, 8) as u8 & KBD_STATUS_IBF == 0 {
            return;
        }
        bus.delay_us(2);
    }
}

/// Pulse the reset line through the keyboard controller, ten times.
pub fn kbd_reset<B: ResetBus>(bus: &mut B) {
    for _ in 0..10 {
        kbd_wait(bus);
        bus.delay_us(50);
        bus.write_io(KBD_STATUS_PORT, 8, KBD_CMD_PULSE_RESET as u32);
        bus.delay_us(50);
    }
}

/// Reset through the PCH reset control register at 0xCF9.
pub fn cf9_reset<B: ResetBus>(bus: &mut B) {
    let keep = (bus.read_io(RST_CNT_PORT, 8) as u8) & !RST_CNT_COLD;
    bus.write_io(RST_CNT_PORT, 8, (keep | RST_CNT_SYS_RST) as u32);
    bus.delay_us(50);
    bus.write_io(RST_CNT_PORT, 8, (keep | RST_CNT_COLD) as u32);
    bus.delay_us(50);
}

/// Every method in turn. Returns only if none of them reset the machine; the
/// caller's last resort is a triple fault.
pub fn reset_sequence<B: ResetBus>(bus: &mut B, fadt: Option<&FadtInfo>) {
    for _ in 0..2 {
        if let Some(f) = fadt {
            acpi_reset(bus, f);
        }
        kbd_reset(bus);
    }
    cf9_reset(bus);
}
