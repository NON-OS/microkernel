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
//! SAE commits (802.11-2020, 12.4.5.4) against group 19's field and order
//! as the standard gives them: a sound commit reads as built; a group not
//! offered, a scalar of 0, 1 or past the order, an element off the curve,
//! a coordinate past the field, or a body cut short is refused.

use super::rng::{Rng, SEEDS};
use super::sae_sim::station;
use nonos_wifi_core::sae::frame::parse_commit;

pub const ROUNDS: usize = 40;

fn refused(body: &[u8], what: &str) {
    assert!(parse_commit(body).is_none(), "{what}");
}

#[test]
fn commits_read_as_built_and_each_lie_is_refused() {
    for seed in SEEDS {
        let mut rng = Rng::new(seed);
        for _ in 0..ROUNDS {
            let body = station(&mut rng).commit_frame().2;
            let c = parse_commit(&body).expect("sound");
            assert_eq!((&c.scalar_bytes[..], &c.element_bytes[..]), (&body[2..34], &body[34..98]));
            let mut lie = body.clone();
            let g = match rng.next() as u16 {
                19 => 20,
                g => g,
            };
            lie[..2].copy_from_slice(&g.to_le_bytes());
            refused(&lie, "another group");
            for s in [0u8, 1] {
                let mut lie = body.clone();
                lie[2..34].fill(0);
                lie[33] = s;
                refused(&lie, "a trivial scalar");
            }
            // n begins FFFFFFFF 00000000: any scalar with a nonzero fifth
            // octet under four FF octets is past it.
            let mut lie = body.clone();
            lie[2..6].fill(0xff);
            lie[6] = 1 + rng.below(255) as u8;
            refused(&lie, "a scalar past the order");
            // For one x the curve holds y and p - y, of opposite parity as p
            // is odd; a flip of any bit but the lowest keeps the parity, so
            // the point leaves the curve.
            let mut lie = body.clone();
            let k = 1 + rng.below(255);
            lie[97 - k / 8] ^= 1 << (k % 8);
            refused(&lie, "an element off the curve");
            let mut lie = body.clone();
            lie[34..66].fill(0xff);
            refused(&lie, "x past the field");
            refused(&body[..rng.below(98)], "cut short");
        }
    }
}
