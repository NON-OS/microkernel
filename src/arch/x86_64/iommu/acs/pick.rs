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

//! The bridges the ACS pass treats as switch downstream ports (L76), by
//! where they sit as well as what they say (`port_rule`).

extern crate alloc;

use alloc::vec::Vec;

use super::port_rule::{picked, Bridge};
use crate::drivers::pci::config::access::read8;
use crate::drivers::pci::types::{HeaderType, PciDevice, PcieDeviceType as Kind};

/// The Device/Port Type a bridge reports, none when its walk read none.
fn reported(d: &PciDevice) -> Option<u8> {
    d.pcie.map(|p| match p.device_type {
        Kind::Endpoint => 0,
        Kind::LegacyEndpoint => 1,
        Kind::RootPort => 4,
        Kind::UpstreamPort => 5,
        Kind::DownstreamPort => 6,
        Kind::PcieToPciBridge => 7,
        Kind::PciToPcieBridge => 8,
        Kind::RootComplexEndpoint => 9,
        Kind::RootComplexEventCollector => 10,
        Kind::Unknown(v) => v,
    })
}

pub(super) fn downstream_ports(devices: &[PciDevice]) -> Vec<&PciDevice> {
    let secondary =
        |d: &PciDevice| read8(d.address.bus, d.address.device, d.address.function, 0x19).ok();
    let found: Vec<&PciDevice> = devices
        .iter()
        .filter(|d| d.address.segment == 0 && d.header_type == HeaderType::PciToPciBridge)
        .collect();
    let read: Vec<Bridge> =
        found.iter().map(|d| (d.address.bus, secondary(d), reported(d))).collect();
    let pick = picked(&read);
    (0..found.len()).filter(|&at| pick(at)).map(|at| found[at]).collect()
}
