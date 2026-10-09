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

//! Intel RST remapping is counted as Linux ahci_remap_check counts it: only
//! with VSCAP bit 0, only slots REMAP_CAP marks, only NVMe class codes, and
//! only on an ABAR of 512 KiB or more that is mapped past the last slot.

use crate::controller::remap::{may_remap, remapped_nvme, REMAP_CAP, REMAP_DCC, VSCAP};

const NVME: u32 = 0x01_08_02;
const SATA_AHCI: u32 = 0x01_06_01;

#[test]
fn the_registers_sit_where_linux_reads_them() {
    assert_eq!((VSCAP, REMAP_CAP, REMAP_DCC), (0xa4, 0x800, 0x880));
}

#[test]
fn one_remapped_nvme_drive_is_counted() {
    assert_eq!(remapped_nvme(1, 0b001, [NVME, 0, 0]), 1);
    assert_eq!(remapped_nvme(1, 0b101, [NVME, NVME, NVME]), 2);
}

#[test]
fn nothing_counts_without_vscap_bit_0() {
    assert_eq!(remapped_nvme(0, 0b111, [NVME; 3]), 0);
    assert_eq!(remapped_nvme(0xffff_fffe, 0b111, [NVME; 3]), 0);
}

#[test]
fn a_slot_not_in_use_or_not_nvme_is_not_counted() {
    assert_eq!(remapped_nvme(1, 0, [NVME; 3]), 0);
    assert_eq!(remapped_nvme(1, 0b111, [SATA_AHCI, 0, 0xffff_ffff]), 0);
    // Bits past the three slots Linux reads name nothing.
    assert_eq!(remapped_nvme(1, 0xffff_fff8, [NVME; 3]), 0);
}

#[test]
fn only_a_large_abar_mapped_past_the_last_slot_is_read() {
    // QEMU's ICH9 (4 KiB) and a PCH's usual 2 KiB ABAR are never read.
    assert!(!may_remap(4096, 4096));
    assert!(!may_remap(2048, 2048));
    // RST RAID mode: 512 KiB, read when the map reaches 0x980 + 4.
    assert!(may_remap(512 * 1024, 512 * 1024));
    assert!(may_remap(512 * 1024, 0x984));
    assert!(!may_remap(512 * 1024, 0x983));
}
