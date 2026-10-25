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

//! VBoxVGA and VBoxSVGA share 80ee:beef. The record builder puts VBoxVGA's
//! DISPI ports in its first free slot, BAR1, where VBoxSVGA keeps its SVGA II
//! ports; the inventory reads the record after that, as the kernel lists it,
//! and still takes only VBoxSVGA for SVGA II.

use super::attach::attach;
use crate::hardware::broker::{Bar, BarKind, DeviceRecord, BAR_FLAG_FIXED};
use crate::inventory_ids::{classify_device, HardwareFamily};

const MIB: u64 = 1 << 20;

fn listed(vendor: u16, device: u16, bars: &[(BarKind, u64, u64)]) -> DeviceRecord {
    let pci = DeviceRecord { bus_kind: 1, pci_class: 0x03, ..DeviceRecord::empty() };
    let mut rec = DeviceRecord { vendor, device, ..pci };
    for (i, &(kind, base, size)) in bars.iter().enumerate() {
        rec.bars[i] = Bar { base, size, kind: kind as u8, ..Bar::empty() };
    }
    rec.bar_count = bars.len() as u8;
    attach(&mut rec);
    rec
}

#[test]
fn vboxvga_s_dispi_ports_are_not_an_svga_port_window() {
    let rec = listed(0x80EE, 0xBEEF, &[(BarKind::Mmio, 0xE000_0000, 16 * MIB)]);
    let b = rec.bars[1];
    assert_eq!(rec.bar_count, 2, "the DISPI window went in a slot of its own");
    assert_eq!((b.base, b.size, b.kind), (0x1CE, 3, BarKind::Pio as u8));
    assert_eq!(b.flags & BAR_FLAG_FIXED, BAR_FLAG_FIXED);
    assert_eq!(classify_device(&rec), HardwareFamily::Unknown, "no SVGA II driver for VBoxVGA");
}

#[test]
fn vboxsvga_and_vmware_svga_keep_their_own_port_bars() {
    let vbox = [
        (BarKind::Mmio, 0xE000_0000, 16 * MIB),
        (BarKind::Pio, 0xD000, 16),
        (BarKind::Mmio, 0xF000_0000, 2 * MIB),
    ];
    let rec = listed(0x80EE, 0xBEEF, &vbox);
    assert_eq!((rec.bar_count, rec.bars[1].flags & BAR_FLAG_FIXED), (3, 0), "nothing placed");
    assert_eq!(classify_device(&rec), HardwareFamily::DisplayVmwareSvga);
    let vmware = listed(0x15AD, 0x0405, &[vbox[1], vbox[0], vbox[2]]);
    assert_eq!(classify_device(&vmware), HardwareFamily::DisplayVmwareSvga);
}

#[test]
fn a_port_bar_flagged_fixed_is_never_taken_for_svga_ports() {
    let mut rec = listed(0x15AD, 0x0405, &[(BarKind::Pio, 0xD000, 16), (BarKind::Mmio, 0, MIB)]);
    rec.bars[0].flags |= BAR_FLAG_FIXED;
    assert_eq!(classify_device(&rec), HardwareFamily::Unknown);
}
