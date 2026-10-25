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

//! The keyring's reply and request are wiped once the reply has gone: an
//! export's key or shield material's words must not stay in freed memory.

use crate::server::answer::answer;

const SECRET: [u8; 8] = [0xA5; 8];

#[test]
fn the_reply_is_sent_whole_then_wiped() {
    let mut request = [0x11u8; 16];
    let mut reply = SECRET.to_vec();
    let mut sent = Vec::new();
    answer(&mut request, &mut reply, |r| sent.extend_from_slice(r));
    assert_eq!(sent, SECRET, "the caller gets the reply as it was made");
    assert!(reply.iter().all(|&b| b == 0), "no reply byte survives the send");
}

#[test]
fn the_request_is_wiped_too() {
    let mut request = [0x5Au8; 32];
    let mut reply = vec![1u8; 4];
    answer(&mut request, &mut reply, |_| {});
    assert!(request.iter().all(|&b| b == 0), "no request byte survives the answer");
}

#[test]
fn an_empty_reply_is_still_answered() {
    let mut request = [7u8; 4];
    let mut reply = Vec::new();
    let mut calls = 0;
    answer(&mut request, &mut reply, |r| {
        calls += 1;
        assert!(r.is_empty());
    });
    assert_eq!(calls, 1, "every request gets exactly one reply");
    assert!(request.iter().all(|&b| b == 0));
}
