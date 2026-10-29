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

//! Bytes as steps, a kind byte and a length each, against the wire's server
//! and the guest's listener: frames from the opener or another pid, the guest
//! accepting, writing, reading or closing, and time passing. Every answer is
//! a whole frame, two at most a frame; a connection is its opener's; another
//! pid's frame moves no byte either way (P.10).

use nonos_capsule_linux_proofs::linux::net::serve_one::server::Server;
use nonos_capsule_linux_proofs::linux::net::sock::{Addr, Domain, Proto, Socks};

const AT: Addr = Addr { ip: [127, 0, 0, 1], port: 5225 };
const GUEST: u32 = 40;
const OPENER: u32 = 70;

pub fn run(b: &[u8]) {
    let (mut t, mut s, mut guest, mut now) = (Socks::new(), Server::new(AT), None, 1_000);
    let l = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
    let at = t.get_mut(l).unwrap();
    (at.local, at.listening, at.backlog) = (Some(AT), true, 2);
    let (mut rest, mut request) = (b, 0);
    while let [kind, n, tail @ ..] = rest {
        let (step, next) = tail.split_at(usize::from(*n).min(tail.len()));
        (rest, request) = (next, request + 1);
        match (kind % 8, guest) {
            (0, _) => guest = guest.or_else(|| t.accept(l, GUEST).ok().map(|(end, _)| end)),
            (1, Some(end)) => _ = t.write(end, step),
            (2, Some(end)) => _ = t.read(end, 64, false),
            (3, Some(end)) => {
                t.release(end, GUEST);
                guest = None;
            }
            (1..=3, None) => {}
            (4, _) => {
                now += u64::from(*n) * 40;
                s.settle(&mut t, now);
            }
            (k, _) => ask(&mut s, &mut t, OPENER + u32::from(k == 7), request, step, now),
        }
        s.out.drain(..).for_each(|(_, f)| crate::whole(&f));
    }
}

fn ask(s: &mut Server, t: &mut Socks, from: u32, request: u64, frame: &[u8], now: u64) {
    let opener = |s: &Server| s.conn.as_ref().map(|c| (c.id, c.pid));
    /* The bytes waiting in the door end for a READ, and in the guest's end. */
    let held = |s: &Server, t: &Socks| {
        let door = s.conn.as_ref().and_then(|c| t.get(c.end));
        let guest = door.and_then(|d| d.peer).and_then(|p| t.get(p));
        (door.map(|d| d.rx.len()), guest.map(|g| g.rx.len()))
    };
    let (before, had, said) = (opener(s), held(s, t), s.out.len());
    s.ask(t, from, request, frame, now);
    assert!(s.out.len() - said <= 2, "more than two answers to one frame");
    let after = opener(s);
    assert!(after.is_none() || after == before || after.is_some_and(|c| c.1 == from));
    if before.is_some_and(|(_, pid)| pid != from) {
        assert_eq!(held(s, t), had, "another pid's frame moved bytes");
    }
}
