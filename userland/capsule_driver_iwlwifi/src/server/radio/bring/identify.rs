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

//! The card named: its register window checked, the NIC taken (which makes
//! the RF id readable) and the firmware chosen by MAC and RF type.

use crate::driver::Driver;
use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::regs::{CSR_HW_REV, CSR_HW_RF_ID, CSR_MSIX_HW_INT_MASK_AD};
use crate::firmware::gen3::select::{select, transport, Image, Refusal, Transport};
use crate::firmware::gen3::start::start_hw;

use super::super::clock::Uptime;
use super::super::say::Line;
use super::seen::Seen;

/// The register window must reach the MSI-X cause and mask registers.
const WINDOW_MIN: u64 = (CSR_MSIX_HW_INT_MASK_AD + 4) as u64;

pub(super) fn identify(d: &Driver, ids: &mut Seen) -> Result<(Image, Transport), Failure> {
    // An adapter that is not an SO platform is left exactly as setup left it.
    if transport(d.pci_device).is_none() {
        return Err(Failure::Refused(Refusal::NotSoDevice(d.pci_device)));
    }
    if d.mmio_len < WINDOW_MIN {
        return Err(Failure::Window);
    }
    ids.touched = true;
    let m = d.regs;
    let mut c = Uptime;
    ids.hw_rev = m.read32(CSR_HW_REV);
    if ids.hw_rev == u32::MAX {
        return Err(Failure::DeadMmio);
    }
    start_hw(&m, &mut c).map_err(Failure::Start)?;
    ids.rf_id = m.read32(CSR_HW_RF_ID);
    Line::new().text(b"pci ").hex(u32::from(d.pci_device)).send();

    select(d.pci_device, ids.hw_rev, ids.rf_id).map_err(Failure::Refused)
}
