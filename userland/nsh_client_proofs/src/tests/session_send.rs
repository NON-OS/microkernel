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

//! What the client sends. Nothing before `AUTH_OK`; afterwards each body
//! is held to the server's rules, and one it would refuse is not sent and
//! does not end the session. One escalation at a time. After `CLOSE`, or
//! once the shell has exited, no more input.

use crate::kit::grant;
use crate::kit::pair::Pair;
use nsh_client::Refusal;
use nsh_wire::{Kind, Signal, WireError};

#[test]
fn nothing_can_be_sent_before_the_session() {
    let mut p = Pair::good();
    assert_eq!(p.client.input(b"id\n"), Err(Refusal::NotNow));
    assert_eq!(p.client.window(80, 24), Err(Refusal::NotNow));
    assert_eq!(p.client.signal(Signal::End), Err(Refusal::NotNow));
    assert_eq!(p.client.escalate(&grant::escalation()), Err(Refusal::NotNow));
    assert_eq!(p.client.close(), Err(Refusal::NotNow));
    assert_eq!(p.client.take_output().len(), 4 + 1 + 1258, "only the hello");
    let mut p = Pair::good();
    p.to_auth();
    assert_eq!(p.client.input(b"id\n"), Err(Refusal::NotNow), "nor between USER_AUTH and AUTH_OK");
    assert!(p.client.take_output().is_empty());
}

#[test]
fn the_session_frames_go_out_as_the_server_reads_them() {
    let mut p = Pair::good();
    p.login();
    p.client.input(&[b'a'; 20_000]).unwrap();
    p.client.window(1000, 1).unwrap();
    p.client.signal(Signal::Interrupt).unwrap();
    let sent = p.sent();
    let kinds: Vec<u8> = sent.iter().map(|f| f.0).collect();
    let want = [Kind::Input, Kind::Input, Kind::Window, Kind::Signal].map(Kind::byte);
    assert_eq!(kinds, want);
    assert_eq!((sent[0].1.len(), sent[1].1.len()), (16_384, 3_616));
    assert_eq!(sent[2].1, [0xE8, 3, 1, 0]);
    assert_eq!(sent[3].1, [1]);
}

#[test]
fn a_body_the_server_would_refuse_is_not_sent() {
    let mut p = Pair::good();
    p.login();
    assert_eq!(p.client.window(0, 24), Err(Refusal::Unsendable(WireError::Value)));
    assert_eq!(p.client.window(80, 1001), Err(Refusal::Unsendable(WireError::Value)));
    assert_eq!(p.client.escalate(&grant::base()), Err(Refusal::Unsendable(WireError::Value)));
    assert_eq!(p.client.escalate(b"junk"), Err(Refusal::Unsendable(WireError::Magic)));
    assert!(p.client.take_output().is_empty());
    assert_eq!(p.client.input(b"x"), Ok(()), "the session goes on");
}
