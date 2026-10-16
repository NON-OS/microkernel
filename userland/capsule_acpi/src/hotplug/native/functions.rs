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

//! The functions behind a port: probed on the secondary bus the kernel
//! names when a card arrives (device 0, the only one a port's link
//! carries), and every one the kernel's table holds below the port taken
//! out when it leaves, in eject mode for an eject, which puts back what it
//! took if a driver still holds one.

use super::find::Bdf;
use crate::sys::acpi_slot::{below_port, secondary_bus};
use crate::sys::pci_depart::{depart, Why, EBUSY};
use crate::sys::pci_probe::probe_function;
use alloc::vec::Vec;

/// Function 0 first; a card with no function 0 has no others (PCI Express
/// Base, recalled). The kernel's probe answers absent for any function that
/// does not answer, so a single-function card costs seven quick misses.
pub fn probe(f: Bdf) -> usize {
    let Some(bus) = secondary_bus(f) else { return 0 };
    if probe_function(bus, 0, 0).is_err() {
        return 0;
    }
    1 + (1..8).filter(|x| probe_function(bus, 0, *x).is_ok()).count()
}

pub fn eject(f: Bdf) -> Result<usize, ()> {
    let mut taken: Vec<Bdf> = Vec::new();
    for g in below_port(f) {
        match depart(g.1, g.2, g.3, Why::Eject) {
            Ok(_) => taken.push(g),
            Err(EBUSY) => {
                for t in &taken {
                    let _ = probe_function(t.1, t.2, t.3);
                }
                return Err(());
            }
            Err(_) => {}
        }
    }
    Ok(taken.len())
}

pub fn gone(f: Bdf) -> Vec<u64> {
    below_port(f).into_iter().filter_map(|g| depart(g.1, g.2, g.3, Why::Gone).ok()).collect()
}
