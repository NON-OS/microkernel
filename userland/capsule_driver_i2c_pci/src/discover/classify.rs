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
use nonos_libc::{DeviceRecord, BUS_KIND_ACPI, BUS_KIND_PCI};

use super::defs::{
    ACPI_LPSS_FAMILY, PCI_CLASS_SERIAL_BUS, PCI_CLASS_SIGNAL_PROC, PCI_SUBCLASS_OTHER,
    UNKNOWN_LPSS_FAMILY,
};
use crate::constants::{device_info, INTEL_VENDOR_ID, UNKNOWN_LPSS_CLOCK_HZ};

// The broker's class for an ACPI platform I2C controller record.
const CLASS_SERIAL: u32 = 0x0060;

// Recognise an I2C host controller on either bus. A PCI function matches by
// Intel vendor and a known device id; an ACPI device carries its serial-bus
// class and its source clock (in the bar's aux field) from the kernel's AML
// enumeration. The transfer engine polls, so a missing legacy IRQ line never
// disqualifies a controller.
//
// An Intel function with an id the table does not know is still taken when
// its class is the one LPSS uses (serial bus 0x0c on Sunrise Point and later,
// signal processing 0x11 on Apollo/Gemini Lake, subclass "other"): bring-up
// then reads the LPSS capabilities register and refuses anything that is not
// an I2C function, so a newer platform's UART or SPI is never driven.
pub(super) fn classify(r: &DeviceRecord) -> Option<(&'static str, u32, bool)> {
    if r.bus_kind == BUS_KIND_PCI && r.vendor == INTEL_VENDOR_ID {
        if let Some((family, clock_hz)) = device_info(r.device) {
            return Some((family, clock_hz, false));
        }
        let lpss_class = matches!(r.pci_class, PCI_CLASS_SERIAL_BUS | PCI_CLASS_SIGNAL_PROC)
            && r.pci_subclass == PCI_SUBCLASS_OTHER;
        if lpss_class {
            return Some((UNKNOWN_LPSS_FAMILY, UNKNOWN_LPSS_CLOCK_HZ, false));
        }
    }
    if r.bus_kind == BUS_KIND_ACPI && r.class == CLASS_SERIAL && r.pci_class == PCI_CLASS_SERIAL_BUS {
        return Some((ACPI_LPSS_FAMILY, r.bars[0].aux, true));
    }
    None
}
