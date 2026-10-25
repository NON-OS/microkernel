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

use super::own_id::{own_requester_id, path_to_root, Hop, ROOT_BUS};

#[test]
fn a_device_under_a_second_host_bridge_is_its_own() {
    // Root buses 0 and 0x40; a root port on 0x40 to bus 0x41 (L61).
    let roots = |bus: u8| bus == ROOT_BUS || bus == 0x40;
    let below = |bus: u8| (bus == 0x41).then_some((0x40, Hop::PciePort));
    let path = |bus: u8| path_to_root(bus, roots, below);
    assert_eq!(path(0x40), Some(vec![]));
    assert!(own_requester_id(true, false, path(0x41).as_deref()));
    assert!(own_requester_id(false, false, path(0x40).as_deref()), "on a root bus, any function");
    assert!(!own_requester_id(true, false, path(0x42).as_deref()), "below no known bridge");
}

#[test]
fn every_bus_terminates_within_its_own_number_of_hops() {
    let chain = |bus: u8| Some((bus - 1, Hop::PciePort));
    for bus in 0..=u8::MAX {
        let path = path_to_root(bus, |b| b == ROOT_BUS, chain).expect("a strict chain reaches the root");
        assert_eq!(path.len(), bus as usize);
    }
}
