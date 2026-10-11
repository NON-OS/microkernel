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

//! The adapter reset and handed its rings, every refusal, the scan, and
//! discovery.

use nonos_libc::{Bar, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};
use nonos_scsi::Kind;

use crate::adapter;
use crate::discover::found;
use crate::model::adapter as model;
use crate::model::sim::Unit;
use crate::regs::{Mmio, Regs};

use super::util::{grant, up};

#[test]
fn reset_interrupts_masked_then_one_page_rings() {
    let (u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    let s = m.0.borrow();
    assert_eq!(s.commands[..2], [1, 3], "reset, then SETUP_RINGS");
    assert_eq!(s.intr_mask, 0, "every interrupt masked: the driver polls");
    let va = u.hba.va;
    let w = &s.setup;
    assert_eq!(w.len(), 132);
    assert_eq!((w[0], w[1]), (1, 1), "one page each");
    assert_eq!(w[2] as u64 | (w[3] as u64) << 32, va >> 12, "the rings state page");
    assert_eq!(w[4] as u64 | (w[5] as u64) << 32, (va + 0x1000) >> 12, "the request ring");
    assert_eq!(w[68] as u64 | (w[69] as u64) << 32, (va + 0x2000) >> 12, "the completion ring");
    assert!(w[6..68].iter().chain(&w[70..]).all(|&x| x == 0), "no other pages");
}

#[test]
fn rings_refused_or_sized_otherwise_are_refusals() {
    let m = model(vec![]);
    m.0.borrow_mut().refuse_rings = true;
    let va = grant();
    assert_eq!(adapter::start(&m, va, va), Err("rings refused"));
    let m = model(vec![]);
    m.0.borrow_mut().wrong_log2 = true;
    assert_eq!(adapter::start(&m, va, va), Err("ring sizes not taken"));
}

#[test]
fn the_scan_asks_all_64_targets_and_numbers_by_target_then_lun() {
    let mut t5 = Unit::disk(32, 512);
    t5.luns = vec![0, 1];
    let (u, m) = up(vec![((5, 1), Unit::disk(16, 4096)), ((5, 0), t5), ((2, 0), Unit::cd(10))]);
    let at: Vec<(u16, u16, Kind)> =
        u.disks.iter().flatten().map(|d| (d.at.target, d.at.lun, d.kind)).collect();
    assert_eq!(at, vec![(2, 0, Kind::Cd), (5, 0, Kind::Disk), (5, 1, Kind::Disk)]);
    let s = m.0.borrow();
    let targets: std::collections::BTreeSet<u8> = s.seen.iter().map(|r| r.target).collect();
    assert_eq!(targets.len(), 64);
    assert_eq!(*targets.iter().last().unwrap(), 63);
}

fn rec(bars: &[(u8, u64)]) -> DeviceRecord {
    let mut r = DeviceRecord {
        device_id: 4,
        bus_kind: BUS_KIND_PCI,
        vendor: 0x15AD,
        device: 0x07C0,
        bar_count: bars.len() as u8,
        ..DeviceRecord::default()
    };
    for (i, &(kind, size)) in bars.iter().enumerate() {
        r.bars[i] = Bar { kind, size };
    }
    r
}

#[test]
fn discovery_takes_the_first_memory_bar_of_32_kib() {
    let vmware = rec(&[(BAR_KIND_PIO, 8), (BAR_KIND_MMIO, 0x8000)]);
    assert_eq!(found(&vmware).map(|f| (f.device_id, f.bar)), Some((4, 1)));
    let qemu = rec(&[(BAR_KIND_MMIO, 0x8000)]);
    assert_eq!(found(&qemu).map(|f| f.bar), Some(0));
    assert!(found(&rec(&[(BAR_KIND_MMIO, 0x4000)])).is_none());
    let mut other = qemu;
    other.device = 0x07B0;
    assert!(found(&other).is_none());
}

#[test]
fn the_window_reads_nothing_past_its_end() {
    let mem = [0x1234_5678u32, 0xAABB_CCDD];
    let w = Mmio::new(mem.as_ptr() as u64, 8);
    assert_eq!((w.r32(0), w.r32(4)), (0x1234_5678, 0xAABB_CCDD));
    assert_eq!(w.r32(8), 0);
    assert_eq!(w.r32(2), 0, "unaligned");
}
