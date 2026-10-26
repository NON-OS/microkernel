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

//! The kernel's fixed port windows (`legacy`), the broker's own placing of
//! an IDE function's compatibility-mode channels in its record, run on the
//! record types it names (`device`).

pub mod device;
#[path = "../../../../../src/hardware/broker/legacy/mod.rs"]
mod legacy;

pub use device::{Bar, DeviceRecord};

#[cfg(test)]
mod fixed_flags_tests;

/// The kernel's `legacy::attach` on a PCI record of `class`, `subclass` and
/// `progif` holding `bars`, its count `span`: what the broker lists.
pub fn fill(class: u8, subclass: u8, progif: u8, bars: &mut [Bar; 6], span: &mut u8) {
    let mut rec = DeviceRecord {
        pci_class: class,
        pci_subclass: subclass,
        pci_progif: progif,
        vendor: 0,
        device: 0,
        bar_count: *span,
        bars: *bars,
    };
    legacy::attach(&mut rec);
    (*bars, *span) = (rec.bars, rec.bar_count);
}
