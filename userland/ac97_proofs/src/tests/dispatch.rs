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

//! The request rules: the header echoed, one payload-bearing op, the PCM
//! limits, a full queue, and every op the audio server never sends refused.

use super::util::{ask, request, status, up};
use crate::protocol::MAX_PCM_CHUNK;
use crate::server::Stream;

#[test]
fn a_reply_echoes_the_op_flags_and_id() {
    let (mut d, _ich) = up();
    let r = ask(&mut d, &mut Stream::default(), &request(1, 0xABCD, &[]));
    assert_eq!(&r[0..4], &0x4e48_4441u32.to_le_bytes());
    assert_eq!(&r[6..10], &[1, 0, 5, 0], "op and flags");
    assert_eq!(&r[12..16], &0xABCDu32.to_le_bytes());
    assert_eq!((u32::from_le_bytes(r[16..20].try_into().unwrap()), status(&r)), (4, 0));
}

#[test]
fn a_payload_on_any_op_but_write_is_refused() {
    let (mut d, _ich) = up();
    for op in [1, 8, 9, 10] {
        assert_eq!(status(&ask(&mut d, &mut Stream::default(), &request(op, 1, &[0; 4]))), -22);
    }
}

#[test]
fn a_write_outside_the_limits_is_refused() {
    let (mut d, _ich) = up();
    let mut st = Stream::default();
    assert_eq!(status(&ask(&mut d, &mut st, &request(7, 1, &[]))), -22);
    let big = vec![1u8; MAX_PCM_CHUNK + 4];
    assert_eq!(status(&ask(&mut d, &mut st, &request(7, 1, &big))), -22);
    let mut short = request(7, 1, &[1; 64]);
    short.truncate(short.len() - 1);
    assert_eq!(status(&ask(&mut d, &mut st, &short)), -22, "fewer bytes than it names");
}

#[test]
fn a_full_queue_says_again_and_keeps_what_it_has() {
    let (mut d, _ich) = up();
    let mut st = Stream::default();
    let chunk = vec![3u8; MAX_PCM_CHUNK];
    for _ in 0..16 {
        assert_eq!(status(&ask(&mut d, &mut st, &request(7, 1, &chunk))), 0);
    }
    assert_eq!(status(&ask(&mut d, &mut st, &request(7, 1, &chunk))), -11);
    let mut out = vec![0u8; 0x1_0000];
    assert_eq!(st.q.pop_into(&mut out), 0x1_0000);
}
