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

//! A command the part answers late (plan/50 C1). The wait gives up by the
//! clock and the device is reset, so the chain it was given is dropped, and
//! no later command reuses the staging page or the descriptors it names.
//! A timed-out command used to stay published while the next one rewrote
//! them: the part then ran the stale chain with the new command's bytes,
//! and the new call failed on the stale completion though its command ran.

use super::harness::harness;
use super::part::ring::bytes;
use super::part::{answering, Answer};
use super::wire::request;
use crate::constants::{VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_STAGING_OFFSET};
use crate::device::cmd::hdr::RESP_HDR_LEN;

const RESET: Option<&str> = Some("virtio-gpu: control queue reset after a stalled command");

#[test]
fn a_timed_out_chain_never_runs_with_the_next_commands_bytes() {
    let h = harness();
    let (x, y) = (request(0x0104), request(0x0105));
    assert_eq!(h.queue.submit(&x, RESP_HDR_LEN as u32).err(), Some("virtio-gpu: device timeout"));
    let rings = bytes(&h.region, VQ_DESC_OFFSET, VQ_STAGING_OFFSET + 64);
    // The part wakes late; the next command is refused by name.
    let part = answering(&h.region, Answer::Spec);
    assert_eq!(h.queue.submit(&y, RESP_HDR_LEN as u32).err(), RESET);
    std::thread::sleep(std::time::Duration::from_millis(20));
    for seen in part.seen() {
        assert_ne!(seen, y, "the stale chain ran with the next command's bytes");
    }
    assert_eq!(h.region.wrote16(VQ_AVAIL_OFFSET + 2), 1, "nothing more was published");
    let staged = bytes(&h.region, VQ_STAGING_OFFSET, x.len());
    assert_eq!(staged, x, "the staging page still holds the command it was given");
    let now = bytes(&h.region, VQ_DESC_OFFSET, VQ_STAGING_OFFSET);
    assert_eq!(now[..VQ_AVAIL_OFFSET], rings[..VQ_AVAIL_OFFSET], "no descriptor rewritten");
}

#[test]
fn every_later_command_is_refused_once_the_queue_is_reset() {
    let h = harness();
    assert!(h.queue.submit(&request(0x0104), RESP_HDR_LEN as u32).is_err());
    let _part = answering(&h.region, Answer::Spec);
    for type_ in [0x0100, 0x0103, 0x0105] {
        assert_eq!(h.queue.submit(&request(type_), RESP_HDR_LEN as u32).err(), RESET);
    }
}

const SUBMIT: &str =
    include_str!("../../../capsule_driver_virtio_gpu/src/device/virtqueue/submit.rs");
const WAIT: &str = include_str!("../../../capsule_driver_virtio_gpu/src/device/virtqueue/wait.rs");

// The wait was 8192 yields of 1024 reads, whose length was the processor's
// and the scheduler's; it is a deadline by the uptime clock (plan/50 C7).
#[test]
fn the_wait_is_a_deadline_by_the_clock_not_a_count() {
    assert!(SUBMIT.contains("wait::used_moved(layout, next, expired_after(CTRLQ_MS))"));
    assert!(!WAIT.contains("mk_yield") && !WAIT.contains("YIELD_LIMIT"));
}
