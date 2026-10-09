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

//! Where struct virtio_gpu_config is read on each transport.
//!
//! The field offsets are the structure's own (events_read 0, num_scanouts
//! 8, num_capsets 12) and the register type adds where the structure starts:
//! 0x14 in the legacy window, the device capability's place on the modern
//! one. The legacy window's offsets used to be applied to both, so a modern
//! device was asked for its scanout count 0x1C bytes into a 16-byte
//! structure.

use crate::constants::{GPU_CFG_EVENTS_READ, GPU_CFG_NUM_CAPSETS, GPU_CFG_NUM_SCANOUTS};
use crate::regs::Regs;
use crate::tests::model::{legacy_window, modern_regs, modern_window, DEVICE_AT, QUEUE_SIZE};

#[test]
fn a_modern_device_is_asked_inside_its_device_configuration() {
    let bar = modern_window(0, QUEUE_SIZE);
    bar.present32(DEVICE_AT, 1);
    bar.present32(DEVICE_AT + 8, 2);
    bar.present32(DEVICE_AT + 12, 3);
    let regs = modern_regs(&bar);
    unsafe {
        assert_eq!(regs.config_r32(GPU_CFG_EVENTS_READ), 1);
        assert_eq!(regs.config_r32(GPU_CFG_NUM_SCANOUTS), 2);
        assert_eq!(regs.config_r32(GPU_CFG_NUM_CAPSETS), 3);
    }
}

#[test]
fn a_legacy_device_is_asked_where_its_window_puts_the_structure() {
    let bar = legacy_window(0, QUEUE_SIZE);
    bar.present32(0x14, 4);
    bar.present32(0x1C, 5);
    bar.present32(0x20, 6);
    let regs = Regs::mmio(bar.base());
    unsafe {
        assert_eq!(regs.config_r32(GPU_CFG_EVENTS_READ), 4);
        assert_eq!(regs.config_r32(GPU_CFG_NUM_SCANOUTS), 5);
        assert_eq!(regs.config_r32(GPU_CFG_NUM_CAPSETS), 6);
    }
}

#[test]
#[allow(clippy::assertions_on_constants)] // guarding constant relations is the point
fn every_field_read_lies_inside_the_16_byte_structure() {
    for off in [GPU_CFG_EVENTS_READ, GPU_CFG_NUM_SCANOUTS, GPU_CFG_NUM_CAPSETS] {
        assert!(off + 4 <= 16, "field at {off:#x} past struct virtio_gpu_config");
    }
}
