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

//! What OP_TX_PACKET lets onto the wire: a whole Ethernet header at least,
//! a maximum frame at most, and nothing whose length disagrees with its
//! header, the same contract every wired driver keeps.

use super::drive::{call, driver, status};
use super::harness::Dev;
use crate::constants::{ETH_HEADER_LEN, MAX_ETHERNET_FRAME};
use crate::protocol::{Request, E_INVAL, E_MSGSIZE, HDR_LEN, OP_TX_PACKET, REPLY_MAX};
use crate::server::dispatch::serve;
use crate::server::error::take_sent;

fn status_for(len: usize) -> i32 {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    let (stalled, reply) = call(&mut d, OP_TX_PACKET, &vec![0xEE; len]);
    assert!(!stalled);
    let offered = tx.avail_idx();
    assert_eq!(offered, u16::from(status(reply.as_ref().unwrap()) == 0), "len {len}");
    status(&reply.unwrap())
}

#[test]
fn only_a_whole_header_up_to_a_maximum_frame_is_sent() {
    for len in 0..ETH_HEADER_LEN {
        assert_eq!(status_for(len), E_INVAL, "a {len}-byte body");
    }
    for len in [ETH_HEADER_LEN, 60, MAX_ETHERNET_FRAME] {
        assert_eq!(status_for(len), 0, "a {len}-byte body");
    }
    assert_eq!(status_for(MAX_ETHERNET_FRAME + 1), E_INVAL);
}

#[test]
fn a_body_whose_length_disagrees_with_its_header_is_refused() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    for claimed in [0u32, 59, 61, u32::MAX] {
        let req = Request { op: OP_TX_PACKET, flags: 0, request_id: 1, payload_len: claimed };
        let mut msg = vec![0u8; HDR_LEN];
        crate::protocol::encode_response_header(&mut msg, &req, claimed);
        msg.extend_from_slice(&[0; 60]);
        let _ = take_sent();
        assert!(!serve(1, &mut d, &msg, &mut None, &mut vec![0u8; REPLY_MAX]));
        assert_eq!(status(&take_sent().remove(0).unwrap()), E_MSGSIZE, "claimed {claimed}");
    }
    assert_eq!(tx.avail_idx(), 0, "nothing was sent");
}
