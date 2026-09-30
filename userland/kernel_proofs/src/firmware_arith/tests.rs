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

use super::ioapic::IoApicInfo;
use super::memory_desc::MemoryDescriptor;
use super::percpu::PercpuRegion;
use super::port_snapshot::PortStatsSnapshot;
use super::stats_snapshot::MmioStatsSnapshot;

const TOP_PAGE: u64 = 0xFFFF_FFFF_FFFF_F000;

fn descriptor(start: u64, pages: u64) -> MemoryDescriptor {
    MemoryDescriptor {
        memory_type: 7,
        physical_start: start,
        virtual_start: 0,
        number_of_pages: pages,
        attribute: 0,
    }
}

#[test]
fn memory_descriptors_saturate_instead_of_overflowing() {
    assert_eq!(descriptor(0, (1 << 52) - 1).size_bytes(), TOP_PAGE);
    assert_eq!(descriptor(0, 1 << 52).size_bytes(), u64::MAX);
    assert_eq!(descriptor(TOP_PAGE, 1).end_address(), u64::MAX);
    assert_eq!(descriptor(0x1000, 2).end_address(), 0x3000);
}

#[test]
fn the_last_percpu_page_holds_its_own_bytes() {
    let region = PercpuRegion::new(TOP_PAGE, 0x1000, 0);
    assert!(region.contains(TOP_PAGE));
    assert!(region.contains(u64::MAX));
    assert!(!region.contains(TOP_PAGE - 1));
    assert_eq!(region.end(), u64::MAX);
}

#[test]
fn counts_from_firmware_and_counters_saturate() {
    let chip = IoApicInfo { id: 0, address: 0, gsi_base: u32::MAX - 1 };
    assert_eq!(chip.gsi_max(), u32::MAX);
    let mut stats = MmioStatsSnapshot::new();
    stats.read_operations = u64::MAX;
    stats.write_operations = 1;
    assert_eq!(stats.total_operations(), u64::MAX);
}

#[test]
fn port_statistics_totals_saturate() {
    let bytes = PortStatsSnapshot { bytes_read: u64::MAX, bytes_written: 1, ..Default::default() };
    assert_eq!(bytes.total_bytes(), u64::MAX);
    let ops = PortStatsSnapshot { read_ops: u64::MAX, string_write_ops: 1, ..Default::default() };
    assert_eq!(ops.total_ops(), u64::MAX);
    let small = PortStatsSnapshot {
        read_ops: 1,
        write_ops: 2,
        string_read_ops: 3,
        string_write_ops: 4,
        io_delays: 100,
        ..Default::default()
    };
    assert_eq!(small.total_ops(), 10);
}
