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

//! P.8: either side's close is end of file to the other. CLOSE lets the door
//! end go and the guest reads end of file; the guest's close answers READ
//! with CLOSED once what it wrote is read, a held READ as well.

use super::{ask, frame, listening, opened, Server, AT, GUEST, NOW, OPENER};
use crate::linux::net::sock::Socks;
use nonos_serve_wire::{read_ask, CLOSE, CLOSED, OK, READ};

const POLLIN: u16 = 0x001;

fn read(s: &mut Server, t: &mut Socks, conn: u16, seq: u32, wait: u32) -> Option<(u8, Vec<u8>)> {
    let asked = ask(s, t, OPENER, &frame(READ, conn, &read_ask(seq, 16, wait)));
    asked.map(|(h, body)| (h.status, body))
}

#[test]
fn close_gives_the_guest_end_of_file_and_answers_a_held_read() {
    let (mut t, l) = listening(8);
    let mut s = Server::new(AT);
    let (conn, end) = opened(&mut s, &mut t, l);
    assert_eq!(read(&mut s, &mut t, conn, 1, 5_000), None, "held");
    assert_eq!(ask(&mut s, &mut t, OPENER, &frame(CLOSE, conn, &[])).unwrap().0.status, OK);
    assert_eq!(s.out.pop().map(|(_, f)| super::answered(&f).0.status), Some(CLOSED));
    assert_eq!(t.read(end, 16, false), Ok(vec![]), "the guest reads end of file");
    assert_ne!(t.bits(end).unwrap() & POLLIN, 0);
    assert!(s.conn.is_none());
}

#[test]
fn the_guests_close_answers_read_closed_after_what_it_wrote() {
    let (mut t, l) = listening(8);
    let mut s = Server::new(AT);
    let (conn, end) = opened(&mut s, &mut t, l);
    t.write(end, b"bye").unwrap();
    t.release(end, GUEST);
    assert_eq!(read(&mut s, &mut t, conn, 1, 0), Some((OK, b"bye".to_vec())));
    assert_eq!(read(&mut s, &mut t, conn, 2, 0), Some((CLOSED, vec![])));
    assert_eq!(ask(&mut s, &mut t, OPENER, &frame(CLOSE, conn, &[])).unwrap().0.status, OK);
    let (conn, end) = opened(&mut s, &mut t, l);
    assert_eq!(read(&mut s, &mut t, conn, 1, 5_000), None, "held");
    t.release(end, GUEST);
    s.settle(&mut t, NOW);
    assert_eq!(s.out.pop().map(|(_, f)| super::answered(&f).0.status), Some(CLOSED));
}
