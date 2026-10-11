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

//! The gfx wire is one: the ops and lengths this driver answers are the
//! numbers driver.virtio_gpu0 gives them, so the compositor speaks both.

use crate::protocol as bga;
use crate::virtio_limits as vl;
use crate::virtio_ops as vo;

#[test]
fn every_op_served_has_virtio_gpus_number() {
    let pairs = [
        (bga::OP_HEALTHCHECK, vo::OP_HEALTHCHECK),
        (bga::OP_TRANSFER_TO_HOST, vo::OP_TRANSFER_TO_HOST),
        (bga::OP_SET_SCANOUT, vo::OP_SET_SCANOUT),
        (bga::OP_FLUSH, vo::OP_FLUSH),
        (bga::OP_GET_PRIMARY_SURFACE, vo::OP_GET_PRIMARY_SURFACE),
    ];
    for (b, v) in pairs {
        assert_eq!(b, v);
    }
}

#[test]
fn every_length_is_virtio_gpus() {
    assert_eq!(bga::STATUS_LEN, vl::STATUS_LEN);
    assert_eq!(bga::TRANSFER_TO_HOST_REQ_LEN, vl::TRANSFER_TO_HOST_REQ_LEN);
    assert_eq!(bga::SET_SCANOUT_REQ_LEN, vl::SET_SCANOUT_REQ_LEN);
    assert_eq!(bga::FLUSH_REQ_LEN, vl::FLUSH_REQ_LEN);
    assert_eq!(bga::GET_PRIMARY_SURFACE_RESP_LEN, vl::GET_PRIMARY_SURFACE_RESP_LEN);
    assert_eq!(bga::REQUEST_MAX, bga::HDR_LEN + vl::TRANSFER_TO_HOST_REQ_LEN);
}

#[test]
fn a_frame_round_trips_through_the_shared_codec() {
    let req = bga::Request { op: bga::OP_FLUSH, flags: 0, request_id: 41 };
    let mut f = [0u8; 44];
    bga::response_header(&mut f, &req, 24);
    let (back, body) = bga::parse(&f).expect("parsed");
    assert_eq!((back.op, back.request_id, body.len()), (bga::OP_FLUSH, 41, 24));
    assert!(bga::parse(&f[..43]).is_none(), "a frame shorter than it says");
}
