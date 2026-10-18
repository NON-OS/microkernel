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

//! From the claim to the disks found: the order of the broker calls, the
//! channel resets with the interrupt held down, every slot probed, and a
//! packet device or an empty channel left alone.

use nonos_libc::{calls, logged, Script};

use super::util::{qemu, up, up_with, DEVICE};
use crate::model::Drive;

#[test]
fn claims_then_grants_every_window_then_one_dma32_grant() {
    let (d, _) = up(qemu(4096));
    let c = calls();
    assert_eq!(c[0], ("claim", DEVICE));
    // Bus Master and Interrupt Disable.
    assert_eq!(c[1], ("pci_command", (1 << 2) | (1 << 10)));
    let bars: Vec<u64> = c.iter().filter(|c| c.0 == "pio_grant").map(|c| c.1).collect();
    assert_eq!(bars, vec![4, 0, 1, 2, 3], "the bus master window, then each channel's two");
    assert_eq!(c.iter().find(|c| c.0 == "dma_map").unwrap().1, (1 << 1) | (1 << 2));
    assert!(!c.iter().any(|c| c.0.ends_with("release")), "nothing given back: {c:?}");
    assert_eq!(d.disk_count(), 1);
}

#[test]
fn the_disk_is_unit_0_and_the_cd_drive_is_named_and_left() {
    let (d, p) = up(qemu(4096));
    let disk = d.disk(0).expect("the primary master");
    assert_eq!((disk.channel, disk.drive, disk.id.sectors), (0, 0, 4096));
    assert!(d.disk(2).is_none(), "the CD drive is not served");
    let log = logged();
    assert!(log.iter().any(|l| l.contains("unit 2: a packet device")), "{log:?}");
    // Both channels reset once, with nIEN held through and after.
    let p = p.borrow();
    for ch in &p.ch {
        assert_eq!(ch.resets, 1);
        assert_eq!(ch.devctl, 0x02, "nIEN stays set; the driver polls");
    }
    // The disk's slot is marked DMA capable for firmware that reads it.
    assert_eq!(p.ch[0].bmis & 0x60, 0x20);
}

#[test]
fn every_slot_of_a_full_controller_is_a_unit() {
    let slots = [
        [Some(Drive::disk(100, true)), Some(Drive::disk(200, false))],
        [Some(Drive::disk(300, true)), Some(Drive::disk(400, true))],
    ];
    let (d, p) = up(slots);
    let sizes: Vec<u64> = (0..4).map(|u| d.disk(u).unwrap().id.sectors).collect();
    assert_eq!(sizes, vec![100, 200, 300, 400]);
    assert_eq!(d.disk(1).unwrap().drive, 1);
    assert!(!d.disk(1).unwrap().id.lba48);
    assert_eq!(p.borrow().ch[1].bmis & 0x60, 0x60);
    assert!(d.disk(4).is_none(), "units stop at four");
}

#[test]
fn a_floating_channel_is_not_reset_and_its_slots_are_empty() {
    let slots = [[None, Some(Drive::disk(64, true))], [None, None]];
    let (d, p) = up(slots);
    assert_eq!(d.disk_count(), 1);
    assert_eq!(d.disk(1).unwrap().drive, 1, "a slave alone is found");
    assert_eq!(p.borrow().ch[1].resets, 0, "nothing drives the secondary");
    assert!(p.borrow().ch[1].log.is_empty());
}

#[test]
fn a_controller_with_a_cd_drive_alone_comes_up_with_no_disk() {
    let (d, _) = up([[None, None], [Some(Drive::packet()), None]]);
    assert_eq!(d.disk_count(), 0, "main gives everything back and leaves");
}

#[test]
fn a_native_control_window_is_read_at_its_register() {
    let s = Script { ports: [8, 4, 8, 4, 16], ..Script::default() };
    let (d, _) = up_with(qemu(64), s, 2);
    assert_eq!(d.expect("bring-up").disk_count(), 1);
}

#[test]
fn a_channel_whose_devices_stay_busy_is_skipped_after_the_wait() {
    let slots = [[Some(Drive::disk(64, true)), None], [Some(Drive::disk(64, true)), None]];
    nonos_libc::script(Script::default());
    let p = crate::model::install(slots, 0);
    p.borrow_mut().ch[1].busy_forever = true;
    let d = crate::setup::run(crate::discover::Found { device_id: DEVICE, channels: [true, true] })
        .expect("bring-up");
    assert_eq!(d.disk_count(), 1);
    assert!(d.disk(2).is_none());
    assert!(p.borrow().ch[1].log.is_empty(), "no command to a channel still busy");
}

#[test]
fn a_disk_without_dma_is_named_and_left() {
    let mut nodma = Drive::disk(64, true);
    nodma.identify[49] &= !(1 << 8);
    let (d, _) = up([[Some(nodma), None], [None, None]]);
    assert_eq!(d.disk_count(), 0);
    assert!(logged().iter().any(|l| l.contains("unit 0: not served, no DMA")));
}

#[test]
fn the_console_names_the_model_and_size_and_never_the_serial() {
    let (d, _) = up(qemu(2 * 1024 * 1024));
    crate::say::disk(0, &d.disk(0).unwrap());
    let line = logged().last().unwrap().clone();
    assert_eq!(
        line,
        "driver.piix_ide0: unit 0: QEMU HARDDISK, 2097152 sectors (1024 MiB), LBA48\n"
    );
    assert!(!logged().iter().any(|l| l.contains("QM00001")));
}
