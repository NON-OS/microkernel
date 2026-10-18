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

//! The display wire is virtio-gpu's: every op served, every length and
//! every status carries the number driver.virtio_gpu0 gives it, and a frame
//! either driver heads reads under the other's header, so the compositor
//! speaks to both through one client.

use super::frames::{answered, flush_body, frame};
use super::model::{served, take, COMPOSITOR};
use super::virtio_wire as vw;
use crate::serve::{E_BAD_OP, E_BUSY, E_DEVICE, E_INVAL, FLUSH_LEN, OP_FLUSH};
use crate::serve::{OP_GET_PRIMARY_SURFACE, OP_SET_SCANOUT, OP_TRANSFER_TO_HOST, PRIMARY_LEN};
use crate::serve::{REPLY_LEN, SET_SCANOUT_LEN, TRANSFER_LEN};

#[test]
fn every_op_served_has_virtio_gpus_number() {
    assert_eq!(OP_TRANSFER_TO_HOST, vw::OP_TRANSFER_TO_HOST);
    assert_eq!(OP_SET_SCANOUT, vw::OP_SET_SCANOUT);
    assert_eq!(OP_FLUSH, vw::OP_FLUSH);
    assert_eq!(OP_GET_PRIMARY_SURFACE, vw::OP_GET_PRIMARY_SURFACE);
}

#[test]
fn every_length_and_status_is_virtio_gpus() {
    assert_eq!(TRANSFER_LEN, vw::TRANSFER_TO_HOST_REQ_LEN);
    assert_eq!(SET_SCANOUT_LEN, vw::SET_SCANOUT_REQ_LEN);
    assert_eq!(FLUSH_LEN, vw::FLUSH_REQ_LEN);
    assert_eq!(PRIMARY_LEN, vw::GET_PRIMARY_SURFACE_RESP_LEN);
    assert_eq!(REPLY_LEN, vw::HDR_LEN + vw::STATUS_LEN + vw::GET_PRIMARY_SURFACE_RESP_LEN);
    let virtio = [vw::E_INVAL, vw::E_BUSY, vw::E_BAD_OP, vw::E_DEVICE];
    assert_eq!([E_INVAL, E_BUSY, E_BAD_OP, E_DEVICE], virtio);
}

#[test]
fn a_frame_round_trips_through_virtio_gpus_header() {
    let mut s = served(64, 48);
    take(&mut s);
    // A request headed by virtio-gpu's encoder is one this driver takes.
    let req = vw::Request { op: OP_FLUSH, flags: 3, request_id: 41 };
    let body = flush_body(0, 0, 1, 1);
    let mut f = vec![0; vw::HDR_LEN];
    vw::response_header(&mut f, &req, body.len() as u32);
    f.extend_from_slice(&body);
    let (status, reply) = answered(&mut s.d, COMPOSITOR, &f);
    assert_eq!(status, 0, "a frame virtio-gpu's encoder headed was refused");
    // Its reply reads under virtio-gpu's decoder, as the request it answers.
    let (back, payload) = vw::parse(&reply).expect("the reply did not parse");
    assert_eq!((back.op, back.flags, back.request_id), (OP_FLUSH, 3, 41));
    assert_eq!(payload, &0i32.to_le_bytes()[..]);
    let (_, reply) = answered(&mut s.d, COMPOSITOR, &frame(OP_GET_PRIMARY_SURFACE, 7, &[]));
    let (back, payload) = vw::parse(&reply).expect("get_primary's reply did not parse");
    assert_eq!((back.op, back.request_id), (OP_GET_PRIMARY_SURFACE, 7));
    assert_eq!(payload.len(), vw::STATUS_LEN + PRIMARY_LEN);
    assert!(vw::parse(&reply[..reply.len() - 1]).is_none(), "a frame shorter than it says");
}
