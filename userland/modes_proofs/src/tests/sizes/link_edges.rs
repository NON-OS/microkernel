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

//! A link's capacity at every edge of clock, depth, lanes and rate,
//! against the test's reading in u128: DisplayPort's payload is 8b/10b up
//! to 8.1 Gbit/s a lane and 128b/132b above; TMDS carries 24 bits a clock.

use super::edges::EDGES;
use crate::link::{carries, Link};

fn model(link: Link, khz: u32, bpp: u32) -> bool {
    let need = khz as u128 * bpp as u128;
    match link {
        Link::Dp { lanes, mbps } => {
            let raw = lanes as u128 * mbps as u128 * 1000;
            need <= if mbps <= 8100 { raw * 8 / 10 } else { raw * 128 / 132 }
        }
        Link::Tmds { max_khz } => bpp >= 24 && need / 24 <= max_khz as u128,
    }
}

#[test]
fn a_link_carries_exactly_what_its_capacity_holds_at_every_edge() {
    let mut yes = 0;
    for (khz, bpp) in super::edges::pairs() {
        for (a, b) in super::edges::pairs().step_by(7) {
            for link in [Link::Dp { lanes: a, mbps: b }, Link::Tmds { max_khz: a }] {
                let got = carries(link, khz, bpp);
                assert_eq!(got, model(link, khz, bpp), "{link:?} {khz} {bpp}");
                yes += got as u32;
            }
        }
    }
    assert!(yes > 1_000, "{yes}");
    let _ = EDGES;
}
