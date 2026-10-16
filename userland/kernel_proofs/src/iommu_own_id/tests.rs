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

// Root port 00:1c.0 to bus 1, a switch upstream on 1 to bus 2, its downstream
// port on 2 to bus 3; a PCIe-to-PCI bridge on bus 3 to bus 4; a conventional
// bridge on 4 to bus 5. Bus 9 has no bridge above it.
fn parent(bus: u8) -> Option<(u8, Hop)> {
    match bus {
        1 => Some((0, Hop::PciePort)),
        2 => Some((1, Hop::PciePort)),
        3 => Some((2, Hop::PciePort)),
        4 => Some((3, Hop::Other)),
        5 => Some((4, Hop::Other)),
        _ => None,
    }
}

fn own(bus: u8, is_pcie: bool) -> bool {
    own_requester_id(bus, is_pcie, false, path_to_root(bus, parent).as_deref())
}

#[test]
fn the_root_bus_and_pcie_behind_pcie_ports_are_their_own() {
    assert!(own(ROOT_BUS, false) && own(ROOT_BUS, true));
    assert!(own(1, true) && own(3, true));
    assert_eq!(path_to_root(3, parent), Some(vec![Hop::PciePort; 3]));
}

#[test]
fn a_bridge_that_may_stand_in_makes_the_device_not_its_own() {
    // Behind the PCIe-to-PCI bridge, conventional or not, and further down.
    assert!(!own(4, false) && !own(4, true) && !own(5, false) && !own(5, true));
    // A function with no PCIe capability below PCIe ports alone.
    assert!(!own(1, false));
}

#[test]
fn an_unknown_path_or_a_loop_is_never_its_own() {
    assert!(!own(9, true) && path_to_root(9, parent).is_none());
    let looped = |bus: u8| Some((bus, Hop::PciePort));
    assert!(path_to_root(7, looped).is_none());
    let upward = |bus: u8| Some((bus.wrapping_add(1), Hop::PciePort));
    assert!(path_to_root(7, upward).is_none());
    assert!(!own_requester_id(7, true, false, None));
}

#[test]
fn a_bridge_that_stands_in_for_its_subordinates_is_never_its_own() {
    assert!(!own_requester_id(ROOT_BUS, false, true, Some(&[])));
    assert!(!own_requester_id(3, true, true, path_to_root(3, parent).as_deref()));
}

#[test]
fn every_bus_terminates_within_its_own_number_of_hops() {
    let chain = |bus: u8| Some((bus - 1, Hop::PciePort));
    for bus in 0..=u8::MAX {
        let path = path_to_root(bus, chain).expect("a strict chain reaches the root");
        assert_eq!(path.len(), bus as usize);
    }
}
