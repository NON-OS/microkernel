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

//! Every request body is the client's. Fed hostile ops and bodies, the
//! server answers each request with exactly one well-formed reply (`call`
//! checks the header against the frame) carrying a status it defines, and
//! never panics. Bulk pipes are left out: no model runs them, and a bulk
//! transfer would wait out its 5 s.

use super::bench::Bench;
use super::hc_device::lock;
use crate::protocol::{E_AGAIN, E_BUSY, E_INVAL, E_IO, E_NODEV, E_PIPE, OP_CONFIGURE_BULK};

const HIGH_SPEED: u32 = 3;
const ROUNDS: u32 = 3_000;
const KNOWN: [i32; 7] = [0, E_INVAL, E_IO, E_NODEV, E_AGAIN, E_BUSY, E_PIPE];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[test]
fn hostile_requests_each_get_one_well_formed_reply() {
    let mut b = Bench::new(HIGH_SPEED);
    lock(&b.hc).answer = |s| vec![0x5A; (s.length as usize).min(40)];
    let slot = b.root_device();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for round in 0..ROUNDS {
        let op = match rng.below(4) {
            0 => rng.next() as u16,
            _ => (1 + rng.below(0x22)) as u16,
        };
        if op == OP_CONFIGURE_BULK {
            continue;
        }
        let len = if rng.below(8) == 0 { rng.below(5000) } else { rng.below(12) } as usize;
        let mut body: Vec<u8> = (0..len).map(|_| rng.next() as u8).collect();
        if !body.is_empty() && rng.below(2) == 0 {
            body[0] = slot;
        }
        let r = b.call(op, &body);
        assert!(KNOWN.contains(&r.status), "round {round}: op {op:#x} gave {}", r.status);
    }
}
