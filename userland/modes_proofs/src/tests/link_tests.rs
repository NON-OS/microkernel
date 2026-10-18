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

//! DisplayPort and TMDS link capacity.

use crate::link::{carries, Link};

#[test]
fn four_lanes_of_hbr2_carry_4k60_reduced_at_eight_bits() {
    // 533.25 MHz x 24 = 12.8 Gbps; 4 x 5.4 x 0.8 = 17.28 Gbps.
    assert!(carries(Link::Dp { lanes: 4, mbps: 5400 }, 533_250, 24));
    assert!(!carries(Link::Dp { lanes: 4, mbps: 2700 }, 533_250, 24), "8.64 Gbps");
    assert!(!carries(Link::Dp { lanes: 2, mbps: 5400 }, 533_250, 24));
}

#[test]
fn the_payload_edge_is_exact() {
    // One lane of RBR: 1.62 Gbps less 8b/10b is 1.296 Gbps, 54 MHz at 24.
    assert!(carries(Link::Dp { lanes: 1, mbps: 1620 }, 54_000, 24));
    assert!(!carries(Link::Dp { lanes: 1, mbps: 1620 }, 54_001, 24));
}

#[test]
fn uhbr_rates_lose_only_128b_132b() {
    // 4 x 10 Gbps x 128 / 132 = 38.79 Gbps carries 8K30 at 24 bpp (1188 MHz).
    assert!(carries(Link::Dp { lanes: 4, mbps: 10_000 }, 1_188_000, 24));
    assert!(carries(Link::Dp { lanes: 4, mbps: 10_000 }, 1_616_000, 24));
    assert!(!carries(Link::Dp { lanes: 4, mbps: 10_000 }, 1_617_000, 24));
}

#[test]
fn tmds_scales_with_deep_colour() {
    let hdmi14 = Link::Tmds { max_khz: 340_000 };
    assert!(carries(hdmi14, 148_500, 24) && carries(hdmi14, 148_500, 36));
    assert!(!carries(hdmi14, 297_000, 30), "371.25 MHz of characters");
    assert!(!carries(hdmi14, 594_000, 24));
    assert!(carries(Link::Tmds { max_khz: 600_000 }, 594_000, 24));
    assert!(!carries(hdmi14, 148_500, 16), "below 8 bits per colour");
}
