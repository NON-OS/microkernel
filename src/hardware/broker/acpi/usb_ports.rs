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

//! Each USB host controller's port facts (`_UPC`, `_PLD`), as the ACPI
//! capsule read them, kept by the kernel beside the host's record: the
//! record has no room for them, and a host driver or hub walker reads them
//! by the device id it already holds. The capsule names the host by PCI
//! function, the only name for it both sides share.

use super::super::{list, pci_index};
use super::usb_port_wire::decode;
use alloc::vec::Vec;
use spin::Mutex;

/// PCI base class serial bus, subclass USB (PCI Code and Assignment).
const USB_HOST: (u8, u8) = (0x0C, 0x03);

static FACTS: Mutex<Vec<(u64, Vec<u8>)>> = Mutex::new(Vec::new());

#[derive(Debug, PartialEq, Eq)]
pub enum PublishError {
    Malformed,
    /// No function at that address in the device table.
    NoFunction,
    /// The function is not a USB host controller.
    NotUsbHost,
}

/// The broker device id of the function at `bdf`.
pub fn device_of(bdf: (u16, u8, u8, u8)) -> Option<u64> {
    let (seg, bus, dev, func) = bdf;
    let at = |h: &pci_index::PciHandle| {
        let a = h.address;
        (a.segment, a.bus, a.device, a.function) == (seg, bus, dev, func)
    };
    pci_index::all().into_iter().find(at).map(|h| h.device_id)
}

/// Keeps (or replaces) a host's facts, answering its device id.
pub fn publish(bdf: (u16, u8, u8, u8), bytes: &[u8]) -> Result<u64, PublishError> {
    decode(bytes).ok_or(PublishError::Malformed)?;
    let id = device_of(bdf).ok_or(PublishError::NoFunction)?;
    let record = list().into_iter().find(|r| r.device_id == id);
    if !record.is_some_and(|r| (r.pci_class, r.pci_subclass) == USB_HOST) {
        return Err(PublishError::NotUsbHost);
    }
    let mut facts = FACTS.lock();
    facts.retain(|(d, _)| *d != id);
    facts.push((id, bytes.to_vec()));
    Ok(id)
}

/// The facts a host's ports were published with, in wire form.
pub fn facts(device_id: u64) -> Option<Vec<u8>> {
    FACTS.lock().iter().find(|(d, _)| *d == device_id).map(|(_, b)| b.clone())
}
