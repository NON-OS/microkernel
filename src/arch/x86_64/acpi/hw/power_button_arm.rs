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

//! Arming the polled power button: PWRBTN_EN set, with the SCI kept masked.
//!
//! ACPI 6.5 section 4.8.1.1 has a fixed-feature status bit latch whatever its
//! enable bit says, and `power_button` polls on that. QEMU's PM1 does not:
//! `acpi_pm1_evt_power_down` (hw/acpi/core.c) sets PWRBTN_STS only while
//! PWRBTN_EN is set, so with the enable clear a press is never seen. Setting
//! the enable is what the spec allows on any machine; it also lets a press
//! raise the SCI, which no kernel handler serves, so the SCI's IO-APIC line
//! is masked first, and arming is left alone while a capsule holds the line.

use super::gas::{Gas, SPACE_SYSTEM_IO};
use super::sleep::pm1_halves;

/// PM1 enable bit 8: the power button may raise the SCI.
pub const PM1_EN_PWRBTN: u16 = 1 << 8;

/// One step of arming, in the order it is taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmStep {
    /// Mask the SCI's IO-APIC line.
    MaskSci,
    /// Write this to the PM1 enable register(s).
    Enable(u16),
}

/// The steps that arm the poll, given whether a capsule holds the SCI line
/// and what the PM1 enable register reads now. `None` while a capsule holds
/// the line: the line and the enables are then that capsule's. Otherwise the
/// line is masked before the enable is written, so no press can raise an SCI
/// that is not yet masked, and every enable firmware set is kept.
pub const fn arm(sci_held: bool, enable_now: u16) -> Option<[ArmStep; 2]> {
    if sci_held {
        return None;
    }
    Some([ArmStep::MaskSci, ArmStep::Enable(enable_now | PM1_EN_PWRBTN)])
}

/// The PM1 enable register's port when it can be reached with one 16-bit
/// I/O cycle, as the status register is: the second half of the event block.
pub fn io_enable_port(evt: &Gas) -> Option<u16> {
    let (_, en) = pm1_halves(evt);
    let ok = en.space == SPACE_SYSTEM_IO
        && en.bit_width == 16
        && en.bit_offset == 0
        && en.address != 0
        && en.address <= u16::MAX as u64;
    ok.then_some(en.address as u16)
}
