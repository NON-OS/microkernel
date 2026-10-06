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

//! A drain reads until a read is empty, and asks again for a lost reply.

use crate::browser::net::drain::drain;
use crate::browser::net::{Recv, Source};

struct Script {
    replies: Vec<Recv>,
    reads: usize,
}

impl Source for Script {
    fn recv(&mut self, _handle: u32, out: &mut [u8]) -> Recv {
        self.reads += 1;
        let reply = if self.replies.is_empty() { Recv::Empty } else { self.replies.remove(0) };
        match reply {
            Recv::Bytes(n) => {
                let n = n.min(out.len());
                out[..n].fill(7);
                Recv::Bytes(n)
            }
            other => other,
        }
    }
    fn now_ms(&self) -> i64 {
        0
    }
}

#[test]
fn a_lost_reply_is_asked_for_once_more() {
    let mut s = Script { replies: vec![Recv::Lost, Recv::Bytes(5), Recv::Empty], reads: 0 };
    let mut into = Vec::new();
    let got = drain(&mut s, 1, &mut into, 1 << 20, 25);
    assert_eq!((got.got, into.len(), s.reads), (5, 5, 3));
    let mut s = Script { replies: vec![Recv::Lost, Recv::Lost, Recv::Bytes(5)], reads: 0 };
    assert_eq!(drain(&mut s, 1, &mut Vec::new(), 1 << 20, 25).got, 0, "not twice");
}

#[test]
fn a_drain_stops_at_its_cap_and_says_so() {
    let mut s = Script { replies: vec![Recv::Bytes(6), Recv::Bytes(6)], reads: 0 };
    let mut into = Vec::new();
    let got = drain(&mut s, 1, &mut into, 8, 25);
    assert!(got.full);
    assert_eq!(into.len(), 8);
}
