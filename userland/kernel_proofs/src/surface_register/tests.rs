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

use super::register_rule::{may_register, Refusal};
use crate::device_windows::windows::touches_device_window;

const MMIO_VA: u64 = 0x0000_0080_0000_0000;
const DMA_VA: u64 = 0x0000_00A0_0000_0000;
const HEAP_VA: u64 = 0x0000_0000_4000_0000;

#[test]
fn a_process_s_own_fresh_pages_are_registered() {
    let device = touches_device_window(HEAP_VA, 3 * 4096);
    assert_eq!(may_register(device, &[0x1000, 0x2000, 0x3000], [0x9000, 0xA000]), Ok(()));
}

#[test]
fn device_registers_and_dma_buffers_are_the_broker_s() {
    for va in [MMIO_VA, DMA_VA, MMIO_VA - 4096, DMA_VA + 0x10_0000] {
        let device = touches_device_window(va, 2 * 4096);
        assert_eq!(
            may_register(device, &[0x1000, 0x2000], []),
            Err(Refusal::DeviceWindow),
            "{va:#x}"
        );
    }
}

#[test]
fn a_frame_another_surface_holds_is_not_given_again() {
    // An attached view of another owner's surface, or the same pages twice.
    assert_eq!(may_register(false, &[0x1000, 0x5000], [0x9000, 0x5000]), Err(Refusal::AlreadyHeld));
    assert_eq!(may_register(false, &[0x1000], [0x1000]), Err(Refusal::AlreadyHeld));
}

#[test]
fn one_frame_mapped_twice_in_the_range_is_refused() {
    assert_eq!(may_register(false, &[0x1000, 0x2000, 0x2000], []), Err(Refusal::AlreadyHeld));
}

#[test]
fn a_frame_next_to_a_held_one_is_still_free_to_register() {
    assert_eq!(may_register(false, &[0x2000, 0x4000], [0x1000, 0x3000, 0x5000]), Ok(()));
}

#[test]
fn the_device_window_is_refused_before_anything_is_compared() {
    assert_eq!(may_register(true, &[0x1000], [0x1000]), Err(Refusal::DeviceWindow));
}
