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
use nonos_toolkit::widgets::Surface;

use super::rail_addr::{ipv4_pfx, ipv6_str};
use super::rail_fmt::bps_into;
use super::rail_metric::pair;
use super::rail_text::{head, stat};
use crate::rail::net::Net;

pub use super::rail_geom::net_h as height;

/// The interface as the DHCP client last saw it. The name comes from the sample
/// rather than a literal, so a second interface would caption itself correctly.
pub fn draw(fb: &mut impl Surface, x: u32, y: u32, w: u32, n: &Net) {
    let mut b = [0u8; 64];
    let mut y = head(fb, x, y, w, "Network");
    let state = if n.up { "Up" } else { "Down" };
    y = stat(fb, x, y, w, n.name_str(), state);
    y = stat(fb, x, y, w, "IPv4", ipv4_pfx(&mut b, n.ipv4, n.prefix_len));
    y = stat(fb, x, y, w, "IPv6", ipv6_str(&mut b, n.ipv6));
    stat(fb, x, y, w, "Rates", pair(&mut b, n.rx_bps, n.tx_bps, bps_into));
}
