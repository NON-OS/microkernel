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

//! A reply is read only when it is the one asked for.

use super::server::server_reply;
use crate::linux::net::anon_ops::*;
use crate::linux::net::anon_wire::{arrived, reply, REQUEST_ID};

#[test]
fn a_reply_is_read_only_when_it_is_the_reply_asked_for() {
    let good = server_reply(OP_RECV, E_OK, REQUEST_ID, b"abc");
    assert_eq!(reply(&good, OP_RECV), Some((E_OK, &b"abc"[..])));
    /* Too short to hold a header, at every length. */
    for n in 0..HDR_LEN {
        assert_eq!(reply(&good[..n], OP_RECV), None, "{n} bytes");
    }
    /* Cut inside the body, or with bytes past it: the length is not what came. */
    assert_eq!(reply(&good[..good.len() - 1], OP_RECV), None);
    let mut long = good.clone();
    long.push(0);
    assert_eq!(reply(&long, OP_RECV), None);
    /* Another op, another request, another magic or version. */
    assert_eq!(reply(&good, OP_SEND), None);
    assert_eq!(reply(&server_reply(OP_RECV, E_OK, REQUEST_ID + 1, b"abc"), OP_RECV), None);
    for at in [0, 4] {
        let mut bad = good.clone();
        bad[at] ^= 1;
        assert_eq!(reply(&bad, OP_RECV), None, "byte {at}");
    }
    /* A length past anything net.anon sends, whatever arrived. */
    let mut huge = server_reply(OP_RECV, E_OK, REQUEST_ID, &[]);
    huge[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(reply(&huge, OP_RECV), None);
    assert_eq!(
        reply(&server_reply(OP_RECV, E_OK, REQUEST_ID, &vec![1; PAYLOAD_MAX + 1]), OP_RECV),
        None
    );
    /* What a call returned is believed only inside the buffer. */
    assert_eq!(arrived(-1, 64), None);
    assert_eq!(arrived(65, 64), None);
    assert_eq!(arrived(64, 64), Some(64));
}
