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

//! The configuration reads `own_id` judges by (L51).

use crate::bus::pci::{find_devices, listed_through};
use crate::drivers::pci::capabilities::find_capability;
use crate::drivers::pci::config::access::{read16, read8};

use super::own_id::{own_requester_id, path_to_root, Hop, ROOT_BUS};

/// PCI Express capability ID (PCI Local Bus 3.0, appendix H, recalled).
const CAP_PCIE: u8 = 0x10;

/// Whether `bus:device.function`'s DMA arrives under its own requester id,
/// and no other's under it.
pub(crate) fn own_id(bus: u8, device: u8, function: u8) -> bool {
    let is_pcie = find_capability(bus, device, function, CAP_PCIE).is_some();
    let bridge = read8(bus, device, function, 0x0E).is_ok_and(|h| h & 0x7F == 0x01);
    let stands_in = bridge && kind(bus, device, function) == Hop::Other;
    let path = path_to_root(bus, is_root, parent_of);
    own_requester_id(is_pcie, stands_in, path.as_deref())
}

// A root bus: bus 0, or one no listed bridge's range holds, as a second host
// bridge's (EPYC's 0x20, 0x40; Xeon's stacks; L61), read only where the table
// leaves out no bridge that could hold it (L68).
fn is_root(bus: u8) -> bool {
    // A bridge whose range cannot be read is taken to hold it.
    let holds = |(b, d, f): (u8, u8, u8)| {
        let range = (read8(b, d, f, 0x19), read8(b, d, f, 0x1A));
        !matches!(range, (Ok(sec), Ok(sub)) if bus < sec || bus > sub)
    };
    let bridges = find_devices(0x06, 0x04).chain(find_devices(0x06, 0x09));
    let mut all = bridges.chain(find_devices(0x06, 0x07)).map(|b| (b.bus, b.device, b.function));
    bus == ROOT_BUS || (listed_through(bus) && !all.any(holds))
}

// The PCI-to-PCI bridge (class 06, subclass 04 or 09) whose secondary bus is
// `bus`, as its own bus and its kind.
fn parent_of(bus: u8) -> Option<(u8, Hop)> {
    find_devices(0x06, 0x04)
        .chain(find_devices(0x06, 0x09))
        .find(|b| read8(b.bus, b.device, b.function, 0x19).ok() == Some(bus))
        .map(|b| (b.bus, kind(b.bus, b.device, b.function)))
}

// Device/port type, bits 7:4 of the PCIe Capabilities register (PCIe base
// 7.5.3.2, recalled): 4 root port, 5 switch upstream, 6 switch downstream.
// All three pass the id through. Counted Other (L54), a downstream port kept
// every device below it on the identity entry, which reaches all of memory;
// its ACS controls, in extended space this path cannot read, are a separate
// gap and stay recorded as one (L57).
fn kind(bus: u8, device: u8, function: u8) -> Hop {
    let Some(cap) = find_capability(bus, device, function, CAP_PCIE) else {
        return Hop::Other;
    };
    match read16(bus, device, function, cap.offset as u16 + 2).map(|v| (v >> 4) & 0xF) {
        Ok(4..=6) => Hop::PciePort,
        _ => Hop::Other,
    }
}
