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

//! A range is in a domain only when ranges of that one domain cover all of
//! it: inside one range, across abutting ranges of the same domain, never
//! across two domains or a gap; and a bus is in the domain of the root
//! bridge whose buses hold it.

use super::acpi::numa_node::{bus_domain, node_of, BusRange, NodeError, Region};

fn regions() -> Vec<Region> {
    let r = |base, len, domain| Region { base, len, domain };
    vec![
        r(0, 0x8000_0000, 0),
        r(0x1_0000_0000, 0x1_0000_0000, 0),
        r(0x2_0000_0000, 0x1_0000_0000, 1),
    ]
}

#[test]
fn ranges_inside_one_domain() {
    let m = regions();
    assert_eq!(node_of(&m, 0x1000, 0x1000), Ok(0));
    assert_eq!(node_of(&m, 0x2_8000_0000, 0x1000), Ok(1));
    assert_eq!(node_of(&m, 0x1_FFFF_F000, 0x1000), Ok(0), "the last page of a range");
}

#[test]
fn across_domains_gaps_and_nothing() {
    let m = regions();
    assert_eq!(node_of(&m, 0x1_FFFF_F000, 0x2000), Err(NodeError::Spans));
    assert_eq!(node_of(&m, 0x7FFF_F000, 0x2000), Err(NodeError::Outside), "into the hole");
    assert_eq!(node_of(&m, 0x9000_0000, 0x1000), Err(NodeError::Outside));
    assert_eq!(node_of(&m, 0x1000, 0), Err(NodeError::Outside), "no bytes");
    assert_eq!(node_of(&m, u64::MAX - 0xFFF, 0x2000), Err(NodeError::Outside), "wraps");
    assert_eq!(node_of(&[], 0, 0x1000), Err(NodeError::Outside));
}

#[test]
fn abutting_ranges_of_one_domain_join() {
    let r = |base, len, domain| Region { base, len, domain };
    let split = [r(0x1000, 0x1000, 3), r(0, 0x1000, 3)];
    assert_eq!(node_of(&split, 0x800, 0x1000), Ok(3), "listed out of order");
}

#[test]
fn buses_of_root_bridges() {
    let b = |segment, first, last, domain| BusRange { segment, first, last, domain };
    let ranges = [b(0, 0x00, 0x7F, 0), b(0, 0x80, 0xFF, 1), b(1, 0x00, 0xFF, 2)];
    assert_eq!(bus_domain(&ranges, 0, 0x3A), Some(0));
    assert_eq!(bus_domain(&ranges, 0, 0x80), Some(1));
    assert_eq!(bus_domain(&ranges, 1, 0x05), Some(2));
    assert_eq!(bus_domain(&ranges, 2, 0x05), None);
    assert_eq!(bus_domain(&ranges[..1], 0, 0x80), None);
}
