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

//! MkAcpiSlot(port, which, write, value): the call's wire, pure. The
//! kernel, capsule_acpi and the proofs each mount this one file by path.
//! `port` is laid out as MkAcpiPci's address with the register bits 0:
//! segment 32-47, bus 20-27, device 15-19, function 12-14.

/// Registers of a port's PCI Express capability, by `which`.
pub const SLOT_CAPS: u64 = 0;
pub const LINK_STATUS: u64 = 1;
pub const SLOT_CONTROL: u64 = 2;
pub const SLOT_STATUS: u64 = 3;
/// Answers from the kernel's enumeration, not from a register: the
/// `value`th port that may hot plug (`port` 0), the `value`th function
/// below `port`, and `port`'s secondary bus.
pub const PORT_AT: u64 = 16;
pub const BELOW_AT: u64 = 17;
pub const SECONDARY_BUS: u64 = 18;
/// Set in a PORT_AT answer's low bits when the port reports its data link
/// layer state, so the capsule never reads Link Capabilities itself.
pub const DLL_REPORTING: u64 = 1;

pub type Bdf = (u16, u8, u8, u8);

/// Offset in the PCI Express capability and width of each register.
pub fn register(which: u64) -> Option<(u16, u8)> {
    match which {
        SLOT_CAPS => Some((0x14, 4)),
        LINK_STATUS => Some((0x12, 2)),
        SLOT_CONTROL => Some((0x18, 2)),
        SLOT_STATUS => Some((0x1A, 2)),
        _ => None,
    }
}

pub fn pack(f: Bdf) -> u64 {
    let devfn = (u64::from(f.2 & 0x1F) << 15) | (u64::from(f.3 & 7) << 12);
    (u64::from(f.0) << 32) | (u64::from(f.1) << 20) | devfn
}

/// The function a `port` names, or None when any bit outside the four
/// fields is set.
pub fn unpack(port: u64) -> Option<Bdf> {
    if port >> 48 != 0 || port & 0xF000_0FFF != 0 {
        return None;
    }
    let (dev, func) = ((port >> 15) as u8 & 0x1F, (port >> 12) as u8 & 7);
    Some(((port >> 32) as u16, (port >> 20) as u8, dev, func))
}
