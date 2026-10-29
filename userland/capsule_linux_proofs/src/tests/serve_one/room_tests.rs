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

//! P.7: each direction holds what its reader's buffer has room for. A WRITE
//! takes only the room in the guest's end and says how much, 0 when full;
//! the guest's own write to a full door end is EAGAIN, and polls so.

use super::{ask, frame, listening, opened, Server, AT, OPENER};
use crate::linux::abi::errno::EAGAIN;
use nonos_serve_wire::{took_of, write_head, OK, WRITE};

const POLLOUT: u16 = 0x004;
/// A stream's receive buffer (sock/opts.rs).
const ROOM: usize = 131_072;
/// The most bytes one WRITE carries after its number.
const MOST: usize = 65_532;

#[test]
fn write_takes_only_the_room_and_a_guest_write_to_a_full_door_end_is_eagain() {
    let (mut t, l) = listening(8);
    let mut s = Server::new(AT);
    let (conn, end) = opened(&mut s, &mut t, l);
    let mut took = |seq: u32| {
        let body = [&write_head(seq)[..], &[7u8; MOST][..]].concat();
        let (h, n) = ask(&mut s, &mut t, OPENER, &frame(WRITE, conn, &body)).unwrap();
        assert_eq!(h.status, OK);
        took_of(&n).unwrap() as usize
    };
    assert_eq!([took(1), took(2), took(3), took(4)], [MOST, MOST, ROOM - 2 * MOST, 0]);
    assert_eq!(t.get(end).unwrap().rx.len(), ROOM, "the guest's buffer, and no more");
    let door = s.conn.as_ref().unwrap().end;
    assert_eq!(t.write(end, &[1u8; ROOM]), Ok(ROOM));
    assert_eq!(t.write(end, b"x"), Err(EAGAIN), "a guest write to a full door end");
    assert_eq!(t.bits(end).unwrap() & POLLOUT, 0, "and its end polls unwritable");
    assert_eq!(t.read(door, 2, false), Ok(vec![1, 1]));
    assert_eq!(t.write(end, b"xyz"), Ok(2), "what a read made room for");
}
