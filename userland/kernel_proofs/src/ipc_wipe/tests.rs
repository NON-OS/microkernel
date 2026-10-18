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

//! The bytes of a freed payload are zero, whichever way it is freed.

use super::message::IpcMessage;
use super::watch::freed_bytes;
use super::wipe::Payload;

const SECRET: &[u8] = b"a private reply, 32 bytes long..";

fn message() -> IpcMessage {
    IpcMessage::new("proc.9", "endpoint.1", SECRET).unwrap()
}

#[test]
fn the_watch_sees_a_plain_buffer_freed_with_its_bytes() {
    let plain = SECRET.to_vec();
    let p = plain.as_ptr();
    assert_eq!(freed_bytes(p, || drop(plain)), SECRET);
}

#[test]
fn a_dropped_message_is_freed_zeroed() {
    let msg = message();
    let p = msg.data.as_ptr();
    assert_eq!(freed_bytes(p, || drop(msg)), [0; 32]);
}

#[test]
fn a_message_in_a_queue_dropped_whole_is_freed_zeroed() {
    // An inbox removed, or a refused enqueue handing its message back.
    let queue: std::collections::VecDeque<IpcMessage> = [message(), message()].into();
    let p = queue[1].data.as_ptr();
    assert_eq!(freed_bytes(p, || drop(queue)), [0; 32]);
}

#[test]
fn a_shortened_message_is_zeroed_to_its_capacity() {
    let mut msg = message();
    msg.data.truncate(4);
    let p = msg.data.as_ptr();
    assert_eq!(freed_bytes(p, || drop(msg)), [0; 32], "the bytes past len too");
}

#[test]
fn a_copied_in_payload_is_freed_zeroed() {
    let mut data = Payload::zeroed(SECRET.len());
    data.copy_from_slice(SECRET);
    let msg = IpcMessage::new("proc.9", "endpoint.1", &data).unwrap();
    let (p, q) = (data.as_ptr(), msg.data.as_ptr());
    assert_eq!(freed_bytes(p, || drop(data)), [0; 32], "the syscall's copy");
    assert_eq!(freed_bytes(q, || drop(msg)), [0; 32], "the message's own");
}
