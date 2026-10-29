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

//! P.10: one connection, keyed by its id and by the pid that opened it, and
//! one held READ; a malformed frame is EINVAL and delivers nothing.

use super::{ask, frame, listening, opened, Server, AT, NOW, OPENER};
use nonos_serve_wire::{read_ask, write_head, EBADF, EBUSY, EINVAL};
use nonos_serve_wire::{CLOSE, OK, OPEN, READ, WRITE};

const OTHER: u32 = 71;

#[test]
fn one_connection_keyed_by_its_id_and_its_opener_holds_one_read() {
    let (mut t, l) = listening(8);
    let mut s = Server::new(AT);
    let (conn, end) = opened(&mut s, &mut t, l);
    let status = |s: &mut Server, t: &mut _, from, f: &[u8]| ask(s, t, from, f).map(|a| a.0.status);
    for from in [OPENER, OTHER] {
        assert_eq!(status(&mut s, &mut t, from, &frame(OPEN, 0, &[])), Some(EBUSY), "one conn");
    }
    t.write(end, b"for the opener").unwrap();
    let w = [&write_head(1)[..], b"x"].concat();
    for f in
        [frame(READ, conn, &read_ask(1, 64, 0)), frame(WRITE, conn, &w), frame(CLOSE, conn, &[])]
    {
        assert_eq!(status(&mut s, &mut t, OTHER, &f), Some(EBADF), "another pid's frame");
        assert_eq!(
            status(&mut s, &mut t, OPENER, &[&f[..6], &(conn + 1).to_le_bytes(), &f[8..]].concat()),
            Some(EBADF)
        );
    }
    assert_eq!(t.get(end).unwrap().rx.len(), 0, "nothing delivered");
    assert_eq!(status(&mut s, &mut t, OPENER, &frame(READ, conn, &read_ask(1, 64, 0))), Some(OK));
    assert_eq!(status(&mut s, &mut t, OPENER, &frame(READ, conn, &read_ask(2, 64, 5_000))), None);
    let busy = status(&mut s, &mut t, OPENER, &frame(READ, conn, &read_ask(3, 64, 5_000)));
    assert_eq!(busy, Some(EBUSY), "one held READ");
}

#[test]
fn a_malformed_frame_is_einval_with_nothing_delivered() {
    let (mut t, l) = listening(8);
    let mut s = Server::new(AT);
    let (conn, end) = opened(&mut s, &mut t, l);
    let mut f = frame(WRITE, conn, &[&write_head(1)[..], b"abc"].concat());
    f.push(b'!');
    let (h, _) = ask(&mut s, &mut t, OPENER, &f).unwrap();
    assert_eq!((h.op, h.status, h.conn), (WRITE, EINVAL, conn), "under its op and conn");
    s.ask(&mut t, OPENER, 1, b"NSO1", NOW);
    let (_, short) = s.out.pop().unwrap();
    assert_eq!((&short[..4], short[5]), (&b"NSO1"[..], EINVAL), "a short frame too");
    assert_eq!(t.get(end).unwrap().rx.len(), 0);
}
