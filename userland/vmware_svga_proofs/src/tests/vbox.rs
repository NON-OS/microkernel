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

//! VirtualBox's VBoxSVGA: the same adapter under 80ee:beef with VRAM in
//! BAR0, the ports in BAR1 and the FIFO in BAR2, as DevVGA.cpp lays it out;
//! and VBoxVGA, the same id with no SVGA ports, left alone.

use nonos_libc::{calls, Bar, DeviceRecord};

use super::util::firmware;
use crate::bench::Bench;
use crate::constants::pci::{VBOX_LAYOUT, VMWARE_LAYOUT};
use crate::discover::found;

fn record(vendor: u16, kinds: &[(u8, u64)]) -> DeviceRecord {
    let mut r = DeviceRecord { device_id: 6, bus_kind: 1, vendor, device: 0, ..Default::default() };
    r.device = if vendor == 0x80EE { 0xBEEF } else { 0x0405 };
    r.bar_count = kinds.len() as u8;
    for (i, &(kind, size)) in kinds.iter().enumerate() {
        r.bars[i] = Bar { kind, size };
    }
    r
}

#[test]
fn vboxsvga_is_found_in_virtualbox_s_bar_order() {
    let vbox = record(0x80EE, &[(1, 16 << 20), (2, 16), (1, 2 << 20)]);
    let f = found(&vbox).expect("VBoxSVGA");
    assert_eq!((f.layout, f.fb_bytes, f.fifo_bytes), (VBOX_LAYOUT, 16 << 20, 2 << 20));
    let vmware = record(0x15AD, &[(2, 16), (1, 16 << 20), (1, 2 << 20)]);
    assert_eq!(found(&vmware).map(|f| f.layout), Some(VMWARE_LAYOUT));
}

#[test]
fn vboxvga_and_a_mismatched_order_are_left_alone() {
    assert!(found(&record(0x80EE, &[(1, 16 << 20)])).is_none(), "VBoxVGA: no ports");
    let vmware_order = record(0x80EE, &[(2, 16), (1, 16 << 20), (1, 2 << 20)]);
    assert!(found(&vmware_order).is_none());
    let vbox_order = record(0x15AD, &[(1, 16 << 20), (2, 16), (1, 2 << 20)]);
    assert!(found(&vbox_order).is_none());
    let no_ports = record(0x80EE, &[(1, 16 << 20), (1, 16), (1, 2 << 20)]);
    assert!(found(&no_ports).is_none(), "the port window in memory space");
}

#[test]
fn a_vboxsvga_is_brought_up_through_its_own_bars() {
    let mut bench = Bench::standard();
    let f = bench.laid_out(VBOX_LAYOUT, firmware);
    let d = crate::setup::run(f, crate::pool::Pool::default())
        .expect("bring-up through VirtualBox's order");
    assert_eq!((d.server.display.mode.width, d.server.display.mode.height), (1280, 800));
    let grants: Vec<_> = calls().into_iter().filter(|c| c.0 != "claim").collect();
    assert_eq!(grants[..3], [("pio_grant", 1), ("mmio_map", 2), ("mmio_map", 0)]);
}
