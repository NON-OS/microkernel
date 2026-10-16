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

//! The PCI Express capability's slot registers, as offsets from the
//! capability (PCI Express Base, the capability structure chapter; every
//! offset and bit here is recalled, unverified). Pure.

/// Capability id of the PCI Express capability.
pub const CAP_ID: u8 = 0x10;
pub const FLAGS: u16 = 0x02;
pub const LINK_CAP: u16 = 0x0C;
pub const LINK_STATUS: u16 = 0x12;
pub const SLOT_CAP: u16 = 0x14;
pub const SLOT_CTRL: u16 = 0x18;
pub const SLOT_STATUS: u16 = 0x1A;

const SLOT_IMPLEMENTED: u16 = 1 << 8;
const ROOT_PORT: u16 = 4;
const DOWNSTREAM_PORT: u16 = 6;
/// The port says when its data link layer is up.
pub const DLL_REPORTING: u32 = 1 << 20;
pub const DLL_ACTIVE: u16 = 1 << 13;

pub const ATTENTION_BUTTON: u32 = 1 << 0;
pub const POWER_CONTROLLER: u32 = 1 << 1;
pub const HOT_PLUG_CAPABLE: u32 = 1 << 6;
/// The port never sets command completed; nothing is waited for.
pub const NO_COMMAND_COMPLETED: u32 = 1 << 18;

/// Slot control: the power indicator field and power controller off.
pub const POWER_INDICATOR: u16 = 0b11 << 8;
pub const INDICATOR_ON: u16 = 0b01 << 8;
pub const INDICATOR_OFF: u16 = 0b11 << 8;
pub const POWER_OFF: u16 = 1 << 10;

/// Slot status. The change bits are cleared by writing them back as ones.
pub const BUTTON_PRESSED: u16 = 1 << 0;
pub const POWER_FAULT: u16 = 1 << 1;
pub const MRL_CHANGED: u16 = 1 << 2;
pub const PRESENCE_CHANGED: u16 = 1 << 3;
pub const COMMAND_COMPLETED: u16 = 1 << 4;
pub const PRESENT: u16 = 1 << 6;
pub const DLL_CHANGED: u16 = 1 << 8;
pub const CHANGES: u16 =
    BUTTON_PRESSED | POWER_FAULT | MRL_CHANGED | PRESENCE_CHANGED | DLL_CHANGED;

/// A root or downstream port with a slot that can take hot plug.
pub fn hot_plug_port(flags: u16, slot_cap: u32) -> bool {
    let kind = (flags >> 4) & 0xF;
    let port = kind == ROOT_PORT || kind == DOWNSTREAM_PORT;
    port && flags & SLOT_IMPLEMENTED != 0 && slot_cap & HOT_PLUG_CAPABLE != 0
}

/// The slot control word that sets power on or off, its indicator to
/// match, and leaves every other field as it was.
pub fn power_word(ctrl: u16, on: bool) -> u16 {
    let rest = ctrl & !(POWER_INDICATOR | POWER_OFF);
    match on {
        true => rest | INDICATOR_ON,
        false => rest | INDICATOR_OFF | POWER_OFF,
    }
}
