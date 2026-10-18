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

//! The display wire is one: the ops and lengths this driver answers are the
//! numbers driver.virtio_gpu0 gives them, so the compositor speaks both.

use crate::serve as bga;
use crate::virtio_limits as vl;
use crate::virtio_ops as vo;

#[test]
fn every_op_served_has_virtio_gpus_number() {
    let pairs = [
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
    assert_eq!(bga::TRANSFER_LEN, vl::TRANSFER_TO_HOST_REQ_LEN);
    assert_eq!(bga::SET_SCANOUT_LEN, vl::SET_SCANOUT_REQ_LEN);
    assert_eq!(bga::FLUSH_LEN, vl::FLUSH_REQ_LEN);
    assert_eq!(bga::PRIMARY_LEN, vl::GET_PRIMARY_SURFACE_RESP_LEN);
}
