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

//! The boot report of PCIe errors already latched when the kernel enumerates:
//! a link that trained badly, a completion timeout from a Wi-Fi card, a
//! request a bridge refused (UR). Read only. Without an _OSC handshake the
//! firmware keeps AER control (PCI Firmware 3.3, 4.5), so the status bits are
//! left for it to clear.

extern crate alloc;

use alloc::vec::Vec;

use super::decode::{names, CORRECTABLE, COR_STATUS, UNCORRECTABLE, UNCOR_STATUS};
use super::find::aer_offset;
use crate::drivers::pci::config::read_extended32;
use crate::drivers::pci::types::PciDevice;

pub fn report_aer(devices: &[PciDevice]) {
    let (mut read, mut flagged) = (0u32, 0u32);
    for dev in devices.iter().filter(|d| d.pcie.is_some() && d.address.segment == 0) {
        let a = dev.address;
        let Some(cap) = aer_offset(a.bus, a.device, a.function) else { continue };
        let uncor = read_extended32(a.bus, a.device, a.function, cap + UNCOR_STATUS);
        let cor = read_extended32(a.bus, a.device, a.function, cap + COR_STATUS);
        if uncor == !0 || cor == !0 {
            continue;
        }
        read += 1;
        if uncor | cor != 0 {
            flagged += 1;
            log_errors(dev, uncor, cor);
        }
    }
    crate::log::info!("[PCI] AER read on {} functions, {} with errors latched", read, flagged);
}

fn log_errors(dev: &PciDevice, uncor: u32, cor: u32) {
    let u: Vec<&str> = names(uncor, UNCORRECTABLE).collect();
    let c: Vec<&str> = names(cor, CORRECTABLE).collect();
    crate::log::info!(
        "[PCI] {} {:04x}:{:04x} AER uncorrectable {:#010x} {} correctable {:#010x} {}",
        dev.address,
        dev.vendor_id,
        dev.device_id,
        uncor,
        u.join(","),
        cor,
        c.join(",")
    );
}
