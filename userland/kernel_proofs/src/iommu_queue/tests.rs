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

use super::descriptor::{context_global, iec_global, iec_index, iotlb_global, wait};
use super::drain::{read_drain, write_drain};
use super::extended::{extended_interrupt_mode, interrupt_remapping, queued_invalidation};

/* Linux: QI_CC_TYPE 1 | QI_CC_GRAN(DMA_CCMD_GLOBAL_INVL) = 1 << 4. */
#[test]
fn context_global_is_type_one_granularity_global() {
    assert_eq!(context_global(), [0x11, 0]);
}

/* Linux: QI_IOTLB_TYPE 2 | QI_IOTLB_GRAN(DMA_TLB_GLOBAL_FLUSH) = 1 << 4,
QI_IOTLB_DR = bit 7, QI_IOTLB_DW = bit 6, domain id zero for global. */
#[test]
fn iotlb_global_sets_drains_only_when_asked() {
    assert_eq!(iotlb_global(false, false), [0x12, 0]);
    assert_eq!(iotlb_global(true, false), [0x92, 0]);
    assert_eq!(iotlb_global(false, true), [0x52, 0]);
    assert_eq!(iotlb_global(true, true), [0xD2, 0]);
}

/* 6.5.2.7: type 4, G bit 4, IM bits 31:27, IIDX bits 47:32. */
#[test]
fn interrupt_entry_invalidations() {
    assert_eq!(iec_global(), [0x4, 0]);
    assert_eq!(iec_index(0x1234, 3), [0x4 | 0x10 | (3 << 27) | (0x1234 << 32), 0]);
    assert_eq!(iec_index(0xFFFF, 0x1F)[0] >> 48, 0, "nothing above IIDX");
    assert_eq!(iec_index(0, 0xFF)[0], 0x4 | 0x10 | (0x1F << 27), "IM is five bits");
}

/* 6.5.2.8: type 5, SW bit 5, FN bit 6, status data bits 63:32, status
address bits 63:2 in the high quadword. */
#[test]
fn wait_writes_its_sequence_to_a_dword_address() {
    let [low, high] = wait(0x1_2345_6000, 7);
    assert_eq!(low & 0xF, 5);
    assert_ne!(low & (1 << 5), 0, "status write");
    assert_ne!(low & (1 << 6), 0, "fence");
    assert_eq!(low & (1 << 4), 0, "no interrupt");
    assert_eq!(low >> 32, 7);
    assert_eq!(high, 0x1_2345_6000);
    assert_eq!(wait(0x1003, 1)[1], 0x1000, "address bits 1:0 are reserved");
    assert_eq!(wait(0, u32::MAX)[0] >> 32, 0xFFFF_FFFF);
}

#[test]
fn capability_bits_sit_where_the_spec_puts_them() {
    assert!(queued_invalidation(1 << 1) && !queued_invalidation(!(1 << 1)));
    assert!(interrupt_remapping(1 << 3) && !interrupt_remapping(!(1 << 3)));
    assert!(extended_interrupt_mode(1 << 4) && !extended_interrupt_mode(!(1 << 4)));
    assert!(read_drain(1 << 55) && !read_drain(!(1 << 55)));
    assert!(write_drain(1 << 54) && !write_drain(!(1 << 54)));
}
