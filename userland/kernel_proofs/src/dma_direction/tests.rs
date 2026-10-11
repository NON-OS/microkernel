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

use super::direction::{conflicting, device_reads, device_writes};
use super::flags::{
    DMA_MAP_COHERENT, DMA_MAP_DMA32, DMA_MAP_FROM_DEVICE, DMA_MAP_HIGH, DMA_MAP_TO_DEVICE,
    DMA_MAP_WC,
};

const LIBC_DMA: &str = include_str!("../../../libc/src/broker/dma.rs");

#[test]
fn a_grant_with_neither_flag_is_read_and_written() {
    assert!(device_reads(0) && device_writes(0) && !conflicting(0));
}

#[test]
fn to_device_is_read_only_for_the_device() {
    assert!(device_reads(DMA_MAP_TO_DEVICE));
    assert!(!device_writes(DMA_MAP_TO_DEVICE));
}

#[test]
fn from_device_is_write_only_for_the_device() {
    assert!(!device_reads(DMA_MAP_FROM_DEVICE));
    assert!(device_writes(DMA_MAP_FROM_DEVICE));
}

#[test]
fn both_directions_at_once_is_refused() {
    assert!(conflicting(DMA_MAP_TO_DEVICE | DMA_MAP_FROM_DEVICE));
    assert!(!conflicting(DMA_MAP_TO_DEVICE) && !conflicting(DMA_MAP_FROM_DEVICE));
}

#[test]
fn memory_type_and_placement_flags_leave_the_direction_alone() {
    for other in [DMA_MAP_HIGH, DMA_MAP_DMA32, DMA_MAP_COHERENT, DMA_MAP_WC] {
        assert_eq!(other & (DMA_MAP_TO_DEVICE | DMA_MAP_FROM_DEVICE), 0);
        assert!(device_reads(other) && device_writes(other) && !conflicting(other));
        assert!(!device_writes(other | DMA_MAP_TO_DEVICE));
        assert!(!device_reads(other | DMA_MAP_FROM_DEVICE));
    }
}

#[test]
fn libc_names_the_same_bits() {
    assert_eq!(DMA_MAP_TO_DEVICE, 1 << 4);
    assert_eq!(DMA_MAP_FROM_DEVICE, 1 << 5);
    assert!(LIBC_DMA.contains("pub const MK_DMA_MAP_TO_DEVICE: u32 = 1 << 4;"));
    assert!(LIBC_DMA.contains("pub const MK_DMA_MAP_FROM_DEVICE: u32 = 1 << 5;"));
}
