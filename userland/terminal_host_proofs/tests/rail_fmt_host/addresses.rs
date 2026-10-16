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

//! The IPv4 formatter.

use crate::paint::rail_addr::{ipv4_into, ipv4_pfx};
use crate::paint::rail_metric::DASH;
use crate::rail::value::Metric;

#[test]
fn the_ipv4_formatter_writes_a_dotted_quad_with_its_prefix() {
    let mut b = [0u8; 48];
    let n = ipv4_into(&mut b, [10, 0, 2, 15]);
    assert_eq!(core::str::from_utf8(&b[..n]).unwrap(), "10.0.2.15");
    let n = ipv4_into(&mut b, [255, 255, 255, 0]);
    assert_eq!(core::str::from_utf8(&b[..n]).unwrap(), "255.255.255.0");
    assert_eq!(ipv4_pfx(&mut b, Metric::Known([10, 0, 2, 15]), Metric::Known(24)), "10.0.2.15/24");
    assert_eq!(ipv4_pfx(&mut b, Metric::Known([1, 2, 3, 4]), Metric::Unknown), "1.2.3.4");
    assert_eq!(ipv4_pfx(&mut b, Metric::Unknown, Metric::Known(24)), DASH);
}
