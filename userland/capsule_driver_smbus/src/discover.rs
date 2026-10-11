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

//! Finding the SMBus host: an Intel function of class 0Ch subclass 05h whose
//! BAR4 (the datasheets' SMB_BASE) is an I/O range. The iSMT hosts of Atom
//! servers share the class but have no I/O BAR and a DMA interface of their
//! own; requiring the I/O BAR leaves them out.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_PIO};

use crate::regs::IO_SPAN;

const INTEL: u16 = 0x8086;
const CLASS_SERIAL_BUS: u8 = 0x0C;
const SUBCLASS_SMBUS: u8 = 0x05;
/// SMBA, the I/O BAR.
pub const SMB_BAR: u8 = 4;
/// The list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy, Debug)]
pub struct Found {
    pub device_id: u64,
    pub vendor: u16,
    pub device: u16,
}

pub fn find_smbus() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let n = usize::try_from(n).ok()?.min(MAX_DEVICES);
    buf[..n].iter().find(|r| is_i801(r)).map(|r| Found {
        device_id: r.device_id,
        vendor: r.vendor,
        device: r.device,
    })
}

fn is_i801(r: &DeviceRecord) -> bool {
    let bar = r.bars[SMB_BAR as usize];
    r.vendor == INTEL
        && r.pci_class == CLASS_SERIAL_BUS
        && r.pci_subclass == SUBCLASS_SMBUS
        && r.bar_count > SMB_BAR
        && bar.kind == BAR_KIND_PIO
        && bar.size >= IO_SPAN
}
