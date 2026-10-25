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

//! What a resolver, a short header and a display client may hold.

use crate::linux::abi::errno::EMSGSIZE;
use crate::linux::net::dns::limits::{keeps, query_len, MAX_QUERY, MAX_REPLIES};
use crate::wayland_object::{Object, Objects, MAX_OBJECTS};
use crate::wire::{malformed, next};

#[test]
fn a_resolver_takes_one_datagram_and_keeps_a_socket_s_worth() {
    assert_eq!(query_len(MAX_QUERY), Ok(MAX_QUERY as usize));
    assert_eq!(query_len(MAX_QUERY + 1), Err(EMSGSIZE));
    assert_eq!(query_len(u64::MAX), Err(EMSGSIZE), "never sizes a buffer");
    assert!(keeps(MAX_REPLIES - 1) && !keeps(MAX_REPLIES));
}

#[test]
fn a_header_shorter_than_a_header_is_malformed_and_never_waits() {
    let mut buf = vec![1, 0, 0, 0, 0, 0, 4, 0];
    assert!(malformed(&buf) && next(&buf).is_none());
    buf[6] = 8;
    assert!(!malformed(&buf) && next(&buf).is_some());
    buf[6] = 12;
    assert!(!malformed(&buf) && next(&buf).is_none(), "incomplete, not malformed");
    assert!(!malformed(&buf[..7]));
}

#[test]
fn a_display_client_holds_at_most_max_objects_and_may_always_destroy_one() {
    let mut objects = Objects::new();
    // Object 1 is the display; fill the rest.
    for id in 2..=MAX_OBJECTS as u32 {
        assert!(objects.put(id, Object::Callback), "{id}");
    }
    assert!(!objects.put(MAX_OBJECTS as u32 + 1, Object::Buffer), "one more is refused");
    assert!(objects.get(MAX_OBJECTS as u32 + 1).is_none(), "and nothing is recorded");
    assert!(objects.put(7, Object::Buffer), "an id it holds may change what it names");
    assert!(objects.get(7) == Some(Object::Buffer));
    objects.drop_id(7);
    assert!(objects.put(MAX_OBJECTS as u32 + 1, Object::Buffer), "a destroy makes room");
}
