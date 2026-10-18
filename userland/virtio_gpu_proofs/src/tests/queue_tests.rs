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

//! The control queue against a part that consumes it.

use nonos_virtio::common::DEVICE_STATUS;

use super::harness::{harness, DOORBELL};
use super::model::NOTIFY_AT;
use crate::constants::{VG_RESP_OK_NODATA, VRING_DESC_F_NEXT, VRING_DESC_F_WRITE};
use crate::device::cmd::hdr::{Hdr, HDR_LEN, RESP_HDR_LEN};
use crate::tests::part::{answering, Answer};
use crate::tests::wire::request;

const RESET: Option<&str> = Some("virtio-gpu: control queue reset after a stalled command");

#[test]
fn a_command_goes_out_as_a_read_then_write_chain_and_the_answer_is_read_back() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    h.regs.present16(NOTIFY_AT + DOORBELL, 0xFFFF);
    let out = h.queue.submit(&request(0x0104), RESP_HDR_LEN as u32).expect("answered");
    assert_eq!(out.used_len as usize, RESP_HDR_LEN);
    let seen = part.seen.lock().expect("log");
    assert_eq!(seen[0].request, request(0x0104));
    assert_eq!(seen[0].request_flags, VRING_DESC_F_NEXT, "the part reads the request");
    assert_eq!(seen[0].response_flags, VRING_DESC_F_WRITE, "and writes the answer");
    assert_eq!(h.regs.wrote16(NOTIFY_AT + DOORBELL), 0, "the doorbell rang for queue zero");
    let mut resp = [0u8; RESP_HDR_LEN];
    h.queue.read_response(HDR_LEN, &mut resp);
    assert_eq!(Hdr::parse(&resp).map(|r| r.type_), Some(VG_RESP_OK_NODATA));
}

/// A part that answers out of turn, naming another descriptor or moving
/// the used index past the one command published, has the device reset and
/// the queue put out of service: its chain may still name the staging page.
#[test]
fn a_part_that_completes_a_descriptor_the_driver_did_not_publish_is_refused() {
    for answer in [Answer::WrongDescriptor, Answer::AliasedDescriptor, Answer::SkippedIndex] {
        let h = harness();
        h.regs.present8(DEVICE_STATUS, 0x0F);
        let _part = answering(&h.region, answer);
        let err = h.queue.submit(&request(0x0104), RESP_HDR_LEN as u32).err();
        assert_eq!(err, Some("virtio-gpu: used id mismatch"));
        assert_eq!(h.regs.wrote8(DEVICE_STATUS), 0, "the device was reset");
        assert_eq!(h.queue.submit(&request(0x0104), RESP_HDR_LEN as u32).err(), RESET);
    }
}

#[test]
fn a_part_that_never_answers_is_given_up_on_by_the_clock_and_reset() {
    let h = harness();
    h.regs.present8(DEVICE_STATUS, 0x0F);
    let err = h.queue.submit(&request(0x0104), RESP_HDR_LEN as u32).err();
    assert_eq!(err, Some("virtio-gpu: device timeout"));
    assert_eq!(h.regs.wrote8(DEVICE_STATUS), 0, "status 0, read back");
    assert!(h.queue.is_reset(), "a named state, not a stuck slot");
}
