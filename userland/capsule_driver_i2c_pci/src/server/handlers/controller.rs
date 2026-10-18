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

//! OP_CONTROLLER_INFO: who this controller is, as one fixed record (all
//! little-endian; README.md "Interface contract" gives the same table).
//!
//! | bytes  | field                                    |
//! |--------|------------------------------------------|
//! | 0..8   | broker device id                         |
//! | 8..10  | PCI device id (zero for an ACPI device)  |
//! | 10..14 | DesignWare input clock, Hz               |
//! | 14..22 | claim epoch                              |
//! | 22..30 | MMIO grant id                            |
//! | 30..52 | family name, zero padded, cut at 22 bytes |

use crate::driver::Driver;
use crate::protocol::{Request, E_OK};
use crate::server::respond;

pub const INFO_LEN: usize = 52;
const FAMILY_AT: usize = 30;

pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, out: &mut [u8]) {
    let _ = respond::send(sender_pid, req, E_OK, &info(driver), out);
}

fn info(driver: &Driver) -> [u8; INFO_LEN] {
    let mut body = [0u8; INFO_LEN];
    body[0..8].copy_from_slice(&driver.device_id.to_le_bytes());
    body[8..10].copy_from_slice(&driver.pci_device.to_le_bytes());
    body[10..14].copy_from_slice(&driver.setup.clock_hz.to_le_bytes());
    body[14..22].copy_from_slice(&driver.claim_epoch.to_le_bytes());
    body[22..30].copy_from_slice(&driver.mmio_grant.to_le_bytes());
    for (slot, &c) in body[FAMILY_AT..].iter_mut().zip(driver.family.as_bytes()) {
        *slot = c;
    }
    body
}
