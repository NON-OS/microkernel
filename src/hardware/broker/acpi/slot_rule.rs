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

//! MkAcpiSlot's rule, pure: which ports capsule_acpi may drive as native
//! hot-plug slots and which bits it may write (PCI Express Base 6.0,
//! 7.5.3.9 to 7.5.3.11, recalled). A value carrying any other bit is
//! refused, never trimmed, so a capsule bug shows instead of hiding.

use super::slot_wire::{SLOT_CONTROL, SLOT_STATUS};

/// Attention indicator (6-7), power indicator (8-9) and power controller
/// (10). No interrupt enable: the slot is polled and no interrupt of the
/// port is the capsule's. No interlock: nothing here drives one.
pub const CONTROL_BITS: u16 = 0x07C0;
/// The write-1-to-clear events: attention button pressed (0), power fault
/// (1), latch changed (2), presence changed (3), command completed (4),
/// data link layer state changed (8).
pub const STATUS_EVENTS: u16 = 0x011F;

const ROOT_PORT: u16 = 4;
const DOWNSTREAM_PORT: u16 = 6;
const SLOT_IMPLEMENTED: u16 = 1 << 8;
const HOT_PLUG_CAPABLE: u32 = 1 << 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotRefusal {
    /// Slot Capabilities and Link Status are never written.
    ReadOnly,
    /// The value carries a bit outside the register's mask.
    Bits,
}

/// A root port or a switch's downstream port, with a hot-plug capable slot,
/// whose host bridge's `_OSC` gave the OS native hot plug.
pub fn eligible(flags: u16, slot_caps: u32, os_owns_hot_plug: bool) -> bool {
    let kind = (flags >> 4) & 0xF;
    let port = kind == ROOT_PORT || kind == DOWNSTREAM_PORT;
    port && flags & SLOT_IMPLEMENTED != 0 && slot_caps & HOT_PLUG_CAPABLE != 0 && os_owns_hot_plug
}

/// The word to write for `value` when the register holds `now`. Slot
/// Control keeps every bit outside the mask as it is; Slot Status writes 0
/// outside it, which clears nothing and touches no read-only bit.
pub fn write_word(which: u64, value: u32, now: u16) -> Result<u16, SlotRefusal> {
    let (mask, kept) = match which {
        SLOT_CONTROL => (CONTROL_BITS, now & !CONTROL_BITS),
        SLOT_STATUS => (STATUS_EVENTS, 0),
        _ => return Err(SlotRefusal::ReadOnly),
    };
    if value & !u32::from(mask) != 0 {
        return Err(SlotRefusal::Bits);
    }
    Ok(kept | value as u16)
}

/// A function on `bus` is below a port whose secondary and subordinate
/// buses are these; a port with no secondary bus set leads nowhere.
pub fn below(bus: u8, secondary: u8, subordinate: u8) -> bool {
    secondary != 0 && secondary != 0xFF && (secondary..=subordinate).contains(&bus)
}
