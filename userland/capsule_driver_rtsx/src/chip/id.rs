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

//! The ids Linux's rtsx_pci_ids table binds (drivers/misc/cardreader/
//! rtsx_pcr.c), all class FFh "other", and the family each one belongs to
//! here. A reader whose family has no bring-up yet is named in the log and
//! left alone rather than driven with another chip's register values.

pub const VENDOR_REALTEK: u16 = 0x10EC;
const PCI_CLASS_OTHERS: u8 = 0xFF;

pub const LINUX_IDS: [u16; 14] = [
    0x5209, 0x5229, 0x5289, 0x5227, 0x522A, 0x5249, 0x5287, 0x5286, 0x524A, 0x525A, 0x5260, 0x5261,
    0x5228, 0x5264,
];

/// The chip files a reader's settings come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    /// RTS5227 (rts5227_init_params).
    Rts5227,
    /// RTS522A (rts522a_init_params): the RTS5227 with its own PHY values,
    /// register-controlled ASPM and over-current protection.
    Rts522a,
}

pub fn is_linux_reader(vendor: u16, device: u16, pci_class: u8) -> bool {
    vendor == VENDOR_REALTEK && pci_class == PCI_CLASS_OTHERS && LINUX_IDS.contains(&device)
}

/// The family this driver brings up, `None` for a reader it does not.
pub fn family(vendor: u16, device: u16, pci_class: u8) -> Option<Family> {
    if !is_linux_reader(vendor, device, pci_class) {
        return None;
    }
    match device {
        0x5227 => Some(Family::Rts5227),
        0x522A => Some(Family::Rts522a),
        _ => None,
    }
}

/// The BAR holding the host registers: BAR1 on the RTS525A and RTS5264,
/// BAR0 on the rest (rtsx_pci_probe).
pub const fn register_bar(device: u16) -> u32 {
    match device {
        0x525A | 0x5264 => 1,
        _ => 0,
    }
}
