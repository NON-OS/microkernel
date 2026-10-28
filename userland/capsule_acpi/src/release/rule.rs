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

//! Each piece the capsule can run that a power-only build (the
//! `release-power-only` feature) leaves out, since it acts on the machine
//! or the broker beyond what the battery, AC, lid and button path needs.
//! The README's "Release build" says why for each. Pure, proved in
//! capsule_acpi_proofs. Every build loads the tables, finds the EC,
//! connects the regions with their _REG, runs _INI, enters ACPI mode with
//! the GPEs and SCI, and reads batteries, adapters, lids and buttons.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    Pic,         // \_PIC(1): firmware's _PRT choice; nothing uses the routes
    Osc,         // \_SB._OSC and the host bridges' _OSC
    Sleep,       // \_S5 to the kernel's soft-off; PREPARE_SLEEP's \_TTS, \_PTS
    Reserved,    // PNP0C01/PNP0C02 ranges the broker then refuses grants over
    Hidden,      // _PS0 on hidden functions, their probe, D3cold after
    Hotplug,     // ACPI and native hot plug: _DCK, _LCK, _EJ0, departures
    Platform,    // I2C, SD and GPIO hosts published as claimable devices
    BatteryTrip, // _BTP, the battery's trip point written
    PowerOff,    // the capsule's own wiped power-off (button, _CRT, battery)
    FixedButton, // the fixed power button taken from the kernel's poll
    Cooling,     // fans, _SCP, passive limits and the _CRT power-off
    Cpu,         // processor _OSC, _PSS and _CST records for the kernel
    Backlight,   // _BCM and the brightness keys
    HwErrors,    // HEST error blocks read and handed back to firmware
    Ports,       // USB port facts and external-port flags to the broker
    TypeC,       // UCSI: PPM_RESET and the Type-C session
    DevicePower, // D-state moves and runtime wake for drivers
}

/// Whether a build runs `part`: a full build all of them, a power-only
/// build none.
pub const fn runs(power_only: bool, part: Part) -> bool {
    let _ = part;
    !power_only
}

/// The server ops that act, answered EPERM by a power-only build:
/// SET_BRIGHTNESS, PREPARE_SLEEP, DEVICE_POWER, GPIO_EVENT, SET_POWER_CAP.
pub const ACTING_OPS: [u8; 5] = [4, 5, 10, 13, 20];

pub fn op_runs(power_only: bool, op: u8) -> bool {
    !power_only || !ACTING_OPS.contains(&op)
}
