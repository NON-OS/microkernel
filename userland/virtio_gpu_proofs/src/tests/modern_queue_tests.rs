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

//! The bring-up: what the part is told about its rings, and the doorbell.

use nonos_virtio::common::{
    QUEUE_DESC, QUEUE_DEVICE, QUEUE_DRIVER, QUEUE_ENABLE, QUEUE_SELECT, QUEUE_SIZE as Q_SIZE,
};

use super::model::{bring_up, modern_window, regs, NOTIFY_AT, NOTIFY_LEN, NOTIFY_MULT};
use super::model::{NOTIFY_OFF, QUEUE_SIZE, REGION_PHYS};
use crate::constants::{VIRTIO_GPU_F_VIRGL, VQ_AVAIL_OFFSET, VQ_MAX_SIZE, VQ_USED_OFFSET};
use crate::regs::Regs;

fn wrote64(bar: &nonos_devmodel::FakeBar, off: usize) -> u64 {
    bar.wrote32(off) as u64 | (bar.wrote32(off + 4) as u64) << 32
}

#[test]
fn the_rings_are_named_at_the_layout_the_queue_code_uses_and_the_queue_is_enabled() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
    let (_, size, _) = bring_up(&bar, NOTIFY_LEN).expect("live");
    assert_eq!(bar.wrote16(QUEUE_SELECT), 0, "the control queue");
    assert_eq!((bar.wrote16(Q_SIZE), size), (QUEUE_SIZE, QUEUE_SIZE));
    assert_eq!(wrote64(&bar, QUEUE_DESC), REGION_PHYS);
    assert_eq!(wrote64(&bar, QUEUE_DRIVER), REGION_PHYS + VQ_AVAIL_OFFSET as u64);
    assert_eq!(wrote64(&bar, QUEUE_DEVICE), REGION_PHYS + VQ_USED_OFFSET as u64);
    assert_eq!(bar.wrote16(QUEUE_ENABLE), 1);
}

#[test]
fn a_queue_is_cut_to_the_largest_power_of_two_the_region_holds() {
    for (offered, told) in [(1024, VQ_MAX_SIZE), (300, 256), (100, 64), (3, 2), (2, 2)] {
        let bar = modern_window(VIRTIO_GPU_F_VIRGL, offered);
        let (_, size, _) = bring_up(&bar, NOTIFY_LEN).expect("live");
        assert_eq!((size, bar.wrote16(Q_SIZE)), (told, told), "offered {offered}");
    }
}

#[test]
fn a_notification_lands_at_the_offset_the_part_named_scaled_by_the_multiplier() {
    let bar = modern_window(VIRTIO_GPU_F_VIRGL, QUEUE_SIZE);
    let (_, _, doorbell) = bring_up(&bar, NOTIFY_LEN).expect("live");
    let at = NOTIFY_OFF as usize * NOTIFY_MULT as usize;
    assert_eq!(doorbell, at);
    bar.present16(NOTIFY_AT + at, 0xFFFF);
    Regs { doorbell, ..regs(&bar) }.notify(0);
    assert_eq!(bar.wrote16(NOTIFY_AT + at), 0, "queue zero, at the doorbell for queue zero");
}
