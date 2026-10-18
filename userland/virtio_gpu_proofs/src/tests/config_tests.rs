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

//! Where struct virtio_gpu_config is read: at its field offsets (events_read
//! 0, num_scanouts 8, num_capsets 12) inside the device configuration
//! region. The legacy window's offsets used to be applied, so a modern
//! device was asked for its scanout count 0x1C bytes into a 16-byte
//! structure.

use super::model::{modern_window, regs, view, DEVICE_AT, QUEUE_SIZE};
use crate::constants::GPU_CFG_EVENTS_READ;

/// The device configuration's num_scanouts and num_capsets words, as the
/// virtio-gpu layout places them (recalled; unverified). The driver reads
/// neither since G-77: the host's display info names the scanouts.
const GPU_CFG_NUM_SCANOUTS: usize = 0x08;
const GPU_CFG_NUM_CAPSETS: usize = 0x0C;
use crate::regs::Regs;

#[test]
fn a_device_is_asked_inside_its_device_configuration() {
    let bar = modern_window(0, QUEUE_SIZE);
    bar.present32(DEVICE_AT, 1);
    bar.present32(DEVICE_AT + 8, 2);
    bar.present32(DEVICE_AT + 12, 3);
    let regs = regs(&bar);
    assert_eq!(regs.config_r32(GPU_CFG_EVENTS_READ), 1);
    assert_eq!(regs.config_r32(GPU_CFG_NUM_SCANOUTS), 2);
    assert_eq!(regs.config_r32(GPU_CFG_NUM_CAPSETS), 3);
}

#[test]
fn a_field_past_a_short_device_region_reads_all_ones_not_what_follows() {
    let bar = modern_window(0, QUEUE_SIZE);
    bar.present32(DEVICE_AT + 12, 3);
    let short = Regs { device: view(&bar, DEVICE_AT, 12), ..regs(&bar) };
    assert_eq!(short.config_r32(GPU_CFG_NUM_CAPSETS), u32::MAX);
}

#[test]
#[allow(clippy::assertions_on_constants)] // guarding constant relations is the point
fn every_field_read_lies_inside_the_16_byte_structure() {
    for off in [GPU_CFG_EVENTS_READ, GPU_CFG_NUM_SCANOUTS, GPU_CFG_NUM_CAPSETS] {
        assert!(off + 4 <= 16, "field at {off:#x} past struct virtio_gpu_config");
    }
}
