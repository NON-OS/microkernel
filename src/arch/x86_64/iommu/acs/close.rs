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

//! Closing every switch downstream port that can be closed, at boot before
//! any claim (L72), and grouping what sits below a switch that cannot.
//! Only segment 0, through the MCFG window once it is trusted: where it is
//! missing or refused, no port reads as closed and every switch is a group.

extern crate alloc;

use alloc::vec::Vec;

use super::pick::downstream_ports;
use super::port::close;
use crate::drivers::pci::config::access::read8;
use crate::drivers::pci::types::PciDevice;
use crate::hardware::broker::{set_acs_groups, AcsPort as Port};

/// `devices` is the whole boot scan, every bus, so no switch is left half
/// read (L76).
pub(crate) fn close_switches(devices: &[PciDevice]) {
    let mut ports: Vec<Port> = Vec::new();
    for d in downstream_ports(devices) {
        let (bus, dev, f) = (d.address.bus, d.address.device, d.address.function);
        let below = match (read8(bus, dev, f, 0x19), read8(bus, dev, f, 0x1A)) {
            (Ok(first), Ok(last)) => Some((first, last)),
            _ => None,
        };
        ports.push(Port { bus, below, closed: close(bus, dev, f) });
    }
    let groups = set_acs_groups(&ports);
    for &(first, last) in &groups {
        crate::log::info!(
            "[IOMMU] buses {:#x}..={:#x}: below a switch port without ACS, held by one capsule \
             at most; an unclaimed one there is reachable from the others (Decision 4)",
            first,
            last
        );
    }
    let shut = ports.iter().filter(|p| p.closed).count();
    crate::log::info!("[IOMMU] switch ports closed {} of {}", shut, ports.len());
}
