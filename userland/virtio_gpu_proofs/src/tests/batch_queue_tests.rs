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

//! A batch on the control queue against a part that consumes it: every
//! command in its own staging slot and descriptor pair, in order, each
//! answer judged, and a part that answers out of turn has the device
//! reset as a single command's does.

use nonos_virtio::common::DEVICE_STATUS;

use super::harness::harness;
use super::model::QUEUE_SIZE;
use crate::constants::{VG_CMD_TRANSFER_TO_HOST_2D, VQ_STAGING_LEN};
use crate::device::cmd::batch_2d::transfer_rects;
use crate::device::cmd::hdr::RESP_HDR_LEN;
use crate::device::cmd::transfer_to_host_2d::{Rect, REQ_LEN};
use crate::device::virtqueue::batch::{capacity, slot_len};
use crate::state::FenceCounter;
use crate::tests::part::{answering, Answer};
use crate::tests::wire::expected;

fn rects(n: u32) -> Vec<Rect> {
    (0..n).map(|i| Rect { x: i, y: 2 * i, width: 3, height: 4 }).collect()
}

#[test]
fn a_batch_holds_what_the_ring_and_the_staging_page_allow() {
    let h = harness();
    let slot = slot_len(REQ_LEN, RESP_HDR_LEN);
    assert_eq!(slot, 80);
    let cap = capacity(&h.queue, slot);
    assert_eq!(cap, (QUEUE_SIZE as usize / 2).min(VQ_STAGING_LEN / slot));
    assert_eq!(cap, 32);
}

#[test]
fn more_rectangles_than_a_batch_go_out_whole_and_in_order() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    let all = rects(70);
    transfer_rects(&h.queue, &FenceCounter::new(), (5, 100), &mut all.iter().copied())
        .expect("all answered");
    let seen = part.seen();
    assert_eq!(seen.len(), 70);
    for (i, (r, got)) in all.iter().zip(&seen).enumerate() {
        let at = r.y * 400 + r.x * 4;
        let body = [r.x, r.y, 3, 4, at, 0, 5, 0];
        assert_eq!(*got, expected(VG_CMD_TRANSFER_TO_HOST_2D, i as u64 + 1, &body), "{i}");
    }
    assert_eq!(h.queue.published(), 70);
    assert!(!h.queue.is_reset());
}

#[test]
fn empty_rectangles_are_not_sent() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    let mut none = [Rect { x: 0, y: 0, width: 0, height: 9 }].into_iter();
    transfer_rects(&h.queue, &FenceCounter::new(), (5, 100), &mut none).expect("nothing to do");
    assert!(part.seen().is_empty());
    assert_eq!(h.queue.published(), 0);
}

#[test]
fn a_refused_command_fails_the_batch_and_leaves_the_queue_in_step() {
    let h = harness();
    let _part = answering(&h.region, Answer::Reject);
    let got = transfer_rects(&h.queue, &FenceCounter::new(), (5, 100), &mut rects(3).into_iter());
    assert_eq!(got.err(), Some("virtio-gpu: a batched command was rejected"));
    assert_eq!(h.queue.published(), 3, "all three were answered");
    assert!(!h.queue.is_reset());
}

#[test]
fn a_part_answering_out_of_turn_in_a_batch_is_reset() {
    for answer in [Answer::WrongDescriptor, Answer::AliasedDescriptor, Answer::SkippedIndex] {
        let h = harness();
        h.regs.present8(DEVICE_STATUS, 0x0F);
        let _part = answering(&h.region, answer);
        let got = transfer_rects(&h.queue, &FenceCounter::new(), (5, 100), &mut rects(3).into_iter());
        assert_eq!(got.err(), Some("virtio-gpu: used id mismatch"));
        assert_eq!(h.regs.wrote8(DEVICE_STATUS), 0, "the device was reset");
        assert!(h.queue.is_reset());
    }
}

#[test]
fn a_part_that_never_answers_a_batch_is_given_up_on_and_reset() {
    let h = harness();
    h.regs.present8(DEVICE_STATUS, 0x0F);
    let got = transfer_rects(&h.queue, &FenceCounter::new(), (5, 100), &mut rects(4).into_iter());
    assert_eq!(got.err(), Some("virtio-gpu: device timeout"));
    assert!(h.queue.is_reset());
}
