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

//! The bits of the slot registers MkAcpiSlot reads and writes (PCI Express
//! Base, the capability structure chapter; every bit here is recalled,
//! unverified). Offsets and which ports qualify are the kernel's. Pure.

/// Capability id of the PCI Express capability.
pub const DLL_ACTIVE: u16 = 1 << 13;

pub const ATTENTION_BUTTON: u32 = 1 << 0;
pub const POWER_CONTROLLER: u32 = 1 << 1;
pub const NO_COMMAND_COMPLETED: u32 = 1 << 18;

pub const POWER_INDICATOR: u16 = 0b11 << 8;
pub const INDICATOR_ON: u16 = 0b01 << 8;
pub const INDICATOR_OFF: u16 = 0b11 << 8;
pub const POWER_OFF: u16 = 1 << 10;

pub const BUTTON_PRESSED: u16 = 1 << 0;
pub const POWER_FAULT: u16 = 1 << 1;
pub const MRL_CHANGED: u16 = 1 << 2;
pub const PRESENCE_CHANGED: u16 = 1 << 3;
pub const COMMAND_COMPLETED: u16 = 1 << 4;
pub const PRESENT: u16 = 1 << 6;
pub const DLL_CHANGED: u16 = 1 << 8;
pub const CHANGES: u16 =
    BUTTON_PRESSED | POWER_FAULT | MRL_CHANGED | PRESENCE_CHANGED | DLL_CHANGED;

pub fn power_word(ctrl: u16, on: bool) -> u16 {
    let rest = ctrl & !(POWER_INDICATOR | POWER_OFF);
    match on {
        true => rest | INDICATOR_ON,
        false => rest | INDICATOR_OFF | POWER_OFF,
    }
}
