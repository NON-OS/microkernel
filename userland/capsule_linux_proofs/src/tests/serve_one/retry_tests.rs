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

//! P.9: READ and WRITE are numbered. A question asked again under its
//! number, after the asker's call ran out, gets the kept answer, so nothing
//! is taken twice and nothing lost; the next number starts a new exchange.

use super::{ask, frame, listening, opened, Server, AT, NOW, OPENER};
use nonos_serve_wire::{read_ask, took_of, write_head, Header, OK, READ, WRITE};

#[test]
fn a_question_asked_again_under_its_number_gets_the_kept_answer() {
    let (mut t, l) = listening(8);
    let mut s = Server::new(AT);
    let (conn, end) = opened(&mut s, &mut t, l);
    let mut read =
        |t: &mut _, seq| ask(&mut s, t, OPENER, &frame(READ, conn, &read_ask(seq, 64, 0)));
    t.write(end, b"abc").unwrap();
    assert_eq!(read(&mut t, 5).unwrap().1, b"abc");
    t.write(end, b"def").unwrap();
    assert_eq!(read(&mut t, 5).unwrap().1, b"abc", "the kept answer, nothing taken");
    assert_eq!(read(&mut t, 6).unwrap().1, b"def", "the next number reads on");
    let w = [&write_head(9)[..], b"xyz"].concat();
    for _ in 0..2 {
        let (h, n) = ask(&mut s, &mut t, OPENER, &frame(WRITE, conn, &w)).unwrap();
        assert_eq!((h.status, took_of(&n)), (OK, Some(3)));
    }
    assert_eq!(t.read(end, 64, false), Ok(b"xyz".to_vec()), "written once");
}

#[test]
fn a_read_asked_again_while_held_is_answered_to_the_new_call_or_empty_in_time() {
    let (mut t, l) = listening(8);
    let mut s = Server::new(AT);
    let (conn, end) = opened(&mut s, &mut t, l);
    for request in [100, 101] {
        s.ask(&mut t, OPENER, request, &frame(READ, conn, &read_ask(1, 64, 5_000)), NOW);
    }
    t.write(end, b"hi").unwrap();
    s.settle(&mut t, NOW + 1);
    let answers: Vec<_> = s.out.drain(..).map(|(r, f)| (r, super::answered(&f).1)).collect();
    assert_eq!(answers, [(101, b"hi".to_vec())]);
    s.ask(&mut t, OPENER, 102, &frame(READ, conn, &read_ask(2, 64, 50)), NOW);
    s.settle(&mut t, NOW + 49);
    assert!(s.out.is_empty());
    s.settle(&mut t, NOW + 50);
    assert_eq!(
        s.out.pop().map(|(r, f)| (r, super::answered(&f))),
        Some((102, (Header { op: READ, status: OK, conn, len: 0 }, vec![])))
    );
}
