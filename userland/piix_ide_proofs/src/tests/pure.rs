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

//! The pure pieces: the descriptor table, the task file, a request's range,
//! discovery, and the kernel's listing of compatibility-mode channels.

use nonos_libc::{Bar as LBar, DeviceRecord, BAR_KIND_PIO, BUS_KIND_PCI};

use crate::constants::ata::{
    REG_COMMAND, REG_COUNT, REG_DEVICE, REG_LBA_HI, REG_LBA_LO, REG_LBA_MID,
};
use crate::discover::found;
use crate::disk::Flush;
use crate::hardware::broker::{fill, Bar};
use crate::io::prd::build;
use crate::io::{Command, Taskfile};
use crate::server::rw::{parse, Range};

fn entries(t: &[u8], n: usize) -> Vec<(u32, u16, u16)> {
    (0..n)
        .map(|i| {
            let e = &t[i * 8..i * 8 + 8];
            (
                u32::from_le_bytes(e[0..4].try_into().unwrap()),
                u16::from_le_bytes(e[4..6].try_into().unwrap()),
                u16::from_le_bytes(e[6..8].try_into().unwrap()),
            )
        })
        .collect()
}

#[test]
fn prd_regions_stop_at_64_kib_and_the_last_ends_the_table() {
    let mut t = [0u8; 64];
    assert_eq!(build(0x2_0000, 0x1_0000, &mut t), Some(1));
    assert_eq!(entries(&t, 1), vec![(0x2_0000, 0, 0x8000)], "64 KiB is a count of zero");
    assert_eq!(build(0x1_F000, 0x1_0000, &mut t), Some(2));
    assert_eq!(entries(&t, 2), vec![(0x1_F000, 0x1000, 0), (0x2_0000, 0xF000, 0x8000)]);
    assert_eq!(build(0x1000, 512, &mut t), Some(1));
    assert_eq!(entries(&t, 1), vec![(0x1000, 512, 0x8000)]);
}

#[test]
fn prd_refuses_nothing_to_move_a_range_past_4_gib_and_a_short_table() {
    let mut t = [0u8; 64];
    assert_eq!(build(0x1000, 0, &mut t), None);
    assert_eq!(build(0xFFFF_F000, 0x2000, &mut t), None);
    assert_eq!(build(0x1_F000, 0x1_0000, &mut t[..8]), None);
}

#[test]
fn lba28_task_file() {
    let tf = Taskfile::dma(Command::Read, 1, 0x0ABC_DEF1, 256, true).unwrap();
    let want = [
        (REG_COUNT, 0),
        (REG_LBA_LO, 0xF1),
        (REG_LBA_MID, 0xDE),
        (REG_LBA_HI, 0xBC),
        (REG_DEVICE, 0xA0 | 0x10 | 0x40 | 0x0A),
        (REG_COMMAND, 0xC8),
    ];
    assert_eq!(&tf.writes[..tf.n], &want);
}

#[test]
fn lba48_task_file_writes_each_high_byte_first() {
    let tf = Taskfile::dma(Command::Write, 0, 0x1234_5678_9ABC, 0x80, true).unwrap();
    let want = [
        (REG_COUNT, 0),
        (REG_LBA_LO, 0x56),
        (REG_LBA_MID, 0x34),
        (REG_LBA_HI, 0x12),
        (REG_COUNT, 0x80),
        (REG_LBA_LO, 0xBC),
        (REG_LBA_MID, 0x9A),
        (REG_LBA_HI, 0x78),
        (REG_DEVICE, 0xE0),
        (REG_COMMAND, 0x35),
    ];
    assert_eq!(&tf.writes[..tf.n], &want);
}

#[test]
fn task_file_refusals() {
    assert!(
        Taskfile::dma(Command::Read, 0, 1 << 28, 1, false).is_none(),
        "past LBA28 without LBA48"
    );
    assert!(Taskfile::dma(Command::Read, 0, 0, 0, true).is_none());
    assert!(Taskfile::dma(Command::Read, 0, 0, 257, true).is_none());
    assert!(Taskfile::dma(Command::Read, 0, u64::MAX, 1, true).is_none());
    assert!(Taskfile::flush(0, Flush::None).is_none());
    let tf = Taskfile::flush(1, Flush::CacheExt).unwrap();
    assert_eq!(&tf.writes[..tf.n], &[(REG_DEVICE, 0xB0), (REG_COMMAND, 0xEA)]);
}

#[test]
fn request_ranges() {
    let rd = |lba: u64, count: u32| {
        let mut b = lba.to_le_bytes().to_vec();
        b.extend_from_slice(&count.to_le_bytes());
        b
    };
    assert_eq!(parse(&rd(10, 128), false, 138), Ok(Range { lba: 10, count: 128 }));
    assert_eq!(parse(&rd(10, 128), false, 137), Err(-6));
    assert_eq!(parse(&rd(0, 129), false, 1000), Err(-22));
    let mut w = rd(0, 1);
    w.extend_from_slice(&[0; 512]);
    assert_eq!(parse(&w, true, 1), Ok(Range { lba: 0, count: 1 }));
    assert_eq!(parse(&w, false, 1), Err(-90), "a read carries no data");
    assert_eq!(parse(&rd(0, 1), true, 1), Err(-90), "a write carries its sectors");
}

fn piix(device: u16, bars: &[(u8, u16)]) -> DeviceRecord {
    let mut r = DeviceRecord {
        device_id: 3,
        bus_kind: BUS_KIND_PCI,
        pci_class: 1,
        pci_subclass: 1,
        vendor: 0x8086,
        device,
        bar_count: 5,
        ..DeviceRecord::default()
    };
    for &(i, size) in bars {
        r.bars[i as usize] = LBar { kind: BAR_KIND_PIO, size: size as u64 };
    }
    r
}

#[test]
fn discovery_takes_piix3_and_piix4_with_their_windows() {
    let all = [(0, 8), (1, 1), (2, 8), (3, 1), (4, 16)];
    let f = found(&piix(0x7111, &all)).unwrap();
    assert_eq!((f.device_id, f.channels), (3, [true, true]));
    assert!(found(&piix(0x7010, &all)).is_some());
    assert!(found(&piix(0x7113, &all)).is_none(), "the PIIX4 power function is not IDE");
    let f = found(&piix(0x7111, &[(2, 8), (3, 4), (4, 16)])).unwrap();
    assert_eq!(f.channels, [false, true]);
    assert!(found(&piix(0x7111, &[(0, 8), (1, 1)])).is_none(), "no bus master window");
    assert!(found(&piix(0x7111, &[(0, 8), (4, 16)])).is_none(), "no channel whole");
    let mut r = piix(0x7111, &all);
    r.pci_subclass = 6;
    assert!(found(&r).is_none());
}

#[test]
fn the_kernel_lists_compatibility_channels_at_their_isa_ports() {
    let mut bars = [Bar { base: 0, size: 0, kind: 0, flags: 0, aux: 0, _pad: [0; 2] }; 6];
    bars[4] = Bar { base: 0xC000, size: 16, kind: 2, flags: 0, aux: 0, _pad: [0; 2] };
    let mut span = 5;
    fill(1, 1, 0x80, &mut bars, &mut span);
    let got: Vec<(u64, u64, u8)> = bars[..4].iter().map(|b| (b.base, b.size, b.kind)).collect();
    assert_eq!(got, vec![(0x1F0, 8, 2), (0x3F6, 1, 2), (0x170, 8, 2), (0x376, 1, 2)]);
    assert_eq!(span, 5, "the bus master BAR still ends the span");
    assert_eq!(bars[4].base, 0xC000);
}

#[test]
fn a_native_channel_keeps_its_bars_and_other_functions_are_untouched() {
    let native = Bar { base: 0xD000, size: 8, kind: 2, flags: 0, aux: 0, _pad: [0; 2] };
    let mut bars = [native; 6];
    let mut span = 0;
    fill(1, 1, 0x81, &mut bars, &mut span);
    assert_eq!(bars[0], native, "primary native: its BAR stands");
    assert_eq!((bars[2].base, bars[3].base), (0x170, 0x376));
    assert_eq!(span, 4);
    let mut bars = [native; 6];
    let mut span = 1;
    fill(1, 6, 0x00, &mut bars, &mut span);
    fill(2, 1, 0x00, &mut bars, &mut span);
    assert_eq!((bars, span), ([native; 6], 1));
}
