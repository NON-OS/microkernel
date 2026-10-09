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

use super::{bxt, layout};
use crate::constants::{is_bxt_family, HID_INFO_GPIO};
use crate::discover::AcpiTouchpad;
use crate::driver::Doorbell;

/// The touchpad's interrupt line as a register to read, or None when the
/// firmware gave no GpioInt or its layout is not mapped (the HID driver
/// then polls). Broxton keeps its own path: there each community is an
/// ACPI device of its own and the pin is the pad index.
pub fn doorbell(pci_device: u16, tp: &AcpiTouchpad) -> Option<Doorbell> {
    if tp.info & HID_INFO_GPIO == 0 {
        return None;
    }
    if is_bxt_family(pci_device) {
        return bxt::doorbell(pci_device, tp);
    }
    layout::doorbell(tp)
}
