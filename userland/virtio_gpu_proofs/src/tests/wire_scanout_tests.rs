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

//! The wire bytes of the scanout commands, as the part saw them.

use crate::constants::{VG_CMD_RESOURCE_FLUSH, VG_CMD_SET_SCANOUT, VG_CMD_TRANSFER_TO_HOST_2D};
use crate::device::cmd::batch_2d::{flush_rects, transfer_rects};
use crate::device::cmd::set_scanout;
use crate::state::FenceCounter;
use crate::tests::harness::harness;
use crate::tests::part::{answering, Answer};
use crate::tests::wire::{expected, RECT};

#[test]
fn set_scanout_carries_the_rect_then_scanout_then_resource() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    set_scanout(&h.queue, 0, 0, 1, RECT).expect("set");
    assert_eq!(part.seen()[0], expected(VG_CMD_SET_SCANOUT, 0, &[8, 16, 640, 480, 0, 1]));
}

#[test]
fn transfer_to_host_2d_carries_the_rect_offset_resource_and_padding() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    let fences = FenceCounter::new();
    let (_, _) = (fences.issue(), fences.issue());
    // Row 16 of a resource 1024 pixels wide, column 8: 16 * 4096 + 32.
    transfer_rects(&h.queue, &fences, (1, 1024), &mut [RECT].into_iter()).expect("transferred");
    let body = [8, 16, 640, 480, 0x10020, 0, 1, 0];
    let seen = part.seen();
    assert_eq!(seen[0].len(), 56, "virtio_gpu_ctrl_hdr 24, then 32");
    assert_eq!(seen[0], expected(VG_CMD_TRANSFER_TO_HOST_2D, 3, &body));
}

#[test]
fn resource_flush_carries_the_rect_then_resource_and_padding() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    flush_rects(&h.queue, &FenceCounter::new(), 1, &mut [RECT].into_iter()).expect("flushed");
    let seen = part.seen();
    assert_eq!(seen[0].len(), 48, "virtio_gpu_ctrl_hdr 24, then 24");
    assert_eq!(seen[0], expected(VG_CMD_RESOURCE_FLUSH, 1, &[8, 16, 640, 480, 1, 0]));
}
