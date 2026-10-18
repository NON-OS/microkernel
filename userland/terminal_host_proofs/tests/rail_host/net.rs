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

//! Lease decoding and the addresses the rail shows.

use crate::paint::rail_addr::{ipv4_pfx, ipv6_str};
use crate::paint::rail_metric::{pair, DASH};
use crate::rail::net::Net;
use crate::rail::net_decode::decode_lease;
use crate::rail::value::Metric;
use crate::support::lease;
use crate::term::util::format_u64;

#[test]
fn the_lease_the_rail_shows_is_the_lease_the_decoder_read() {
    let mut b = [0u8; 64];
    let n = decode_lease(&lease(3, [10, 0, 2, 15], 24));
    assert_eq!(ipv4_pfx(&mut b, n.ipv4, n.prefix_len), "10.0.2.15/24");
    assert_eq!(ipv6_str(&mut b, n.ipv6), DASH, "no v6 stack, so no v6 row figure");
    assert_eq!(ipv6_str(&mut b, Metric::Known([0; 16])), "0000:0000:0000:0000:0000:0000:0000:0000");
    let rate = pair(&mut b, n.rx_bps, n.tx_bps, |b, v| format_u64(v, b));
    assert_eq!(rate, "— / —", "no byte counters, so each rate is a dash");
}

#[test]
fn a_bound_lease_yields_the_real_address() {
    let n = decode_lease(&lease(3, [10, 0, 2, 15], 24));
    assert!(n.up && n.name_str() == "net0");
    assert_eq!(n.ipv4, Metric::Known([10, 0, 2, 15]));
    assert_eq!(n.prefix_len, Metric::Known(24));
    assert_eq!(n.gateway, Metric::Known([10, 0, 2, 2]));
}

#[test]
fn a_pre_bound_or_short_reply_leaves_the_interface_down() {
    assert_eq!(decode_lease(&lease(2, [10, 0, 2, 15], 24)), Net::DOWN);
    assert_eq!(decode_lease(&[0u8; 8]), Net::DOWN);
    assert!(!Net::DOWN.up && Net::DOWN.ipv4 == Metric::Unknown);
}
