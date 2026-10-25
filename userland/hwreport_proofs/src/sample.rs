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

//! A small report every parser test starts from.

use nonos_hwreport::pci::Function;
use nonos_hwreport::report::Report;

const NIC: Function = Function {
    vendor: 0x8086,
    device: 0x15b8,
    class: 2,
    subclass: 0,
    progif: 0,
    claimed: true,
    external: false,
};

pub fn sample(buf: &mut [u8]) -> (usize, bool) {
    let mut r = Report::new(buf, b"1536ccef3", b"standard-dev", true);
    r.pci(&NIC);
    let mut dsdt = [0x11u8; 44];
    dsdt[..4].copy_from_slice(b"DSDT");
    r.acpi(&dsdt);
    r.log(b"[IOMMU] posture=dma-unconfined reason=no-remapping-hardware-described");
    r.log(b"[WIFI] joined ssid home");
    r.finish()
}
