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

//! Any frame a compositor might send, against a driver's wire server: a
//! frame the test cannot read as a request goes unanswered; every other is
//! answered, and to itself; an op answered 0 is held by `check.rs`.
//! Mounted by each driver's proofs, so every driver is held to the same.

use super::asks::frame;
use super::check::{check, Known};
use super::kit::{Display, Header, Platform, Reply, Server};
use super::payload::payload;
use super::read::request;
use super::rng::Rng;

#[derive(Debug, Default)]
pub struct Tally {
    pub unanswered: usize,
    pub refused: usize,
    pub taken: usize,
    pub shown: usize,
}

pub fn sweep<D: Display, P: Platform>(
    s: &mut Server<D, P>,
    seed: u64,
    rounds: usize,
    owner: u32,
) -> Tally {
    let (mut r, mut t) = (Rng(seed | 1), Tally::default());
    let mut known: [Known; 4] = [(0, (0, 0)); 4];
    for _ in 0..rounds {
        let (op, output) = (r.op(), [0u16, 0, 1, 2, 3, 0xFFFF][r.below(6) as usize]);
        let o = (output as usize).min(3);
        let p = payload(&mut r, op, known[o].0, known[o].1);
        let h = Header { op, flags: 0, output, request_id: r.next() as u32, payload_len: 0 };
        let f = frame(&mut r, h, &p);
        let sender = if r.below(8) == 0 { owner + 1 } else { owner };
        let mut out = [0u8; 300];
        let n = s.handle(sender, &f, &mut out);
        let Some((h, p)) = request(&f) else {
            assert_eq!(n, 0, "a frame that is no request is not answered");
            t.unanswered += 1;
            continue;
        };
        let got = Reply::decode(&out[..n], &h).expect("every request answered, to itself");
        if got.status != 0 {
            t.refused += 1;
            continue;
        }
        t.taken += 1;
        t.shown += check(op, p, got.body, &mut known[o]) as usize;
    }
    t
}
