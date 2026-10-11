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

//! Replies, held to the request they answer.

use super::frame_tests::hdr;
use crate::errno::EINVAL;
use crate::header::{Header, HDR_LEN, MAX_REPLY_BODY};
use crate::ops;
use crate::reply::{reply, Reply};

#[test]
fn a_reply_repeats_the_header_and_is_held_to_the_request() {
    let asked = hdr(ops::STATE, 1, 0);
    let mut out = [0u8; 300];
    let n = reply(&asked, -116, &[9; 32], &mut out).unwrap();
    assert_eq!(n, HDR_LEN + 4 + 32);
    let r = Reply::decode(&out[..n], &asked).unwrap();
    assert_eq!((r.status, r.body, r.header.payload_len), (-116, &[9u8; 32][..], 36));
    let other = Header { request_id: 9, ..asked };
    assert_eq!(Reply::decode(&out[..n], &other), Err(EINVAL), "another request's reply");
    let other = Header { output: 0, ..asked };
    assert_eq!(Reply::decode(&out[..n], &other), Err(EINVAL), "another output's reply");
    assert_eq!(reply(&asked, 0, &[0; MAX_REPLY_BODY + 1], &mut out), Err(EINVAL));
    assert_eq!(reply(&asked, 0, &[0; 8], &mut [0u8; 31]), Err(EINVAL), "no room");
}

#[test]
fn a_reply_without_a_status_or_cut_short_is_refused() {
    let asked = hdr(ops::CAPS, 0, 0);
    let mut out = [0u8; 64];
    let n = reply(&asked, 0, &[1; 32], &mut out).unwrap();
    assert_eq!(Reply::decode(&out[..n - 1], &asked), Err(EINVAL));
    out[16] = 3;
    assert_eq!(Reply::decode(&out[..n], &asked), Err(EINVAL), "length under 4");
}
