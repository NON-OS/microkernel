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

//! The control queue at its edges: a request that cannot go, and a ring
//! that has gone round.

use crate::constants::{VG_CMD_GET_DISPLAY_INFO, VQ_AVAIL_OFFSET, VQ_USED_OFFSET};
use crate::tests::harness::{harness, DOORBELL};
use crate::tests::model::{NOTIFY_AT, QUEUE_SIZE};
use crate::tests::part::{answering, Answer};
use crate::tests::wire::request;

#[test]
fn a_request_that_does_not_fit_the_staging_page_is_refused_before_anything_is_published() {
    let h = harness();
    h.regs.present16(NOTIFY_AT + DOORBELL, 0xFFFF);
    for (req, resp) in [(4000, 200), (4096, 1), (1, 4096), (0, 24), (24, 0)] {
        assert!(h.queue.submit(&vec![0u8; req], resp).is_err(), "{req} + {resp}");
    }
    let err = h.queue.submit(&[0u8; 4000], 200).err();
    assert_eq!(err, Some("virtio-gpu: staging too small"));
    assert_eq!(h.region.wrote16(VQ_AVAIL_OFFSET + 2), 0, "nothing published");
    assert_eq!(h.regs.wrote16(NOTIFY_AT + DOORBELL), 0xFFFF, "no doorbell");
    assert!(!h.queue.is_reset(), "a refused request does not take the device down");
}

#[test]
fn consecutive_commands_advance_both_rings_past_a_wrap() {
    let h = harness();
    let _part = answering(&h.region, Answer::Spec);
    let rounds = QUEUE_SIZE + 6;
    for _ in 0..rounds {
        h.queue.submit(&request(VG_CMD_GET_DISPLAY_INFO), 408).expect("answered");
    }
    assert_eq!(h.region.wrote16(VQ_AVAIL_OFFSET + 2), rounds);
    assert_eq!(h.region.wrote16(VQ_USED_OFFSET + 2), rounds);
}
