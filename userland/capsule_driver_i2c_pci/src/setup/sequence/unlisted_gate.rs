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

use alloc::format;

use nonos_libc::mk_debug;

use crate::constants::device_info;
use crate::discover::Found;
use crate::init::proves_lpss_i2c;
use crate::regs::Regs;

/// An LPSS function the device table does not list is brought up only once
/// its registers read as an I2C core (init/unlisted.rs says why); a refusal
/// is said on the console, naming the PCI id, so the owner can tell a
/// thermal or telemetry function from a new chipset's I2C that needs adding.
pub(super) fn unlisted_gate(dev: &Found, regs: Regs) -> Result<(), &'static str> {
    if !dev.is_lpss() || device_info(dev.pci_device).is_some() {
        return Ok(());
    }
    proves_lpss_i2c(regs).map_err(|why| {
        let s = format!(
            "driver.i2c_pci: Intel {:04x} not listed, left untouched: {}\n",
            dev.pci_device, why
        );
        let _ = mk_debug(s.as_ptr(), s.len());
        "i2c-pci: unlisted Intel function is not an LPSS I2C controller"
    })
}
