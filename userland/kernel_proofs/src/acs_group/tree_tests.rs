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

//! The hops and their peers on the test machine.

use super::acs_path::path;
use super::acs_rule::{FUNCTION, PORT};
use super::machine::*;

#[test]
fn an_endpoint_under_a_switch_climbs_every_port() {
    let hops = path(NIC, &machine()).unwrap();
    let ats: Vec<_> = hops.iter().map(|h| (h.at, h.needed)).collect();
    let want = [(NIC, FUNCTION), (DOWN_A, PORT), (UPSTREAM, PORT), (ROOT_PORT, PORT)];
    assert_eq!(ats, want);
    assert!(hops[0].peers.is_empty(), "a single function");
    assert_eq!(hops[1].peers, [DOWN_B, WIFI], "the other downstream port and its device");
    assert!(hops[2].peers.is_empty(), "the switch's upstream bus has no one else");
    assert_eq!(hops[3].peers, [ROOT_PORT_2, NVME], "the root port's other function");
}

#[test]
fn a_root_port_counts_only_its_own_functions_as_peers() {
    let hops = path(NVME, &machine()).unwrap();
    let root = hops.last().unwrap();
    assert_eq!(root.at, ROOT_PORT_2);
    assert_eq!(root.peers, [ROOT_PORT, UPSTREAM, DOWN_A, DOWN_B, NIC, WIFI], "function 0 of 1c");
}

#[test]
fn a_multi_function_device_names_its_siblings() {
    let hops = path(SATA, &machine()).unwrap();
    assert_eq!(hops.len(), 1, "on the root bus, no port above it");
    assert_eq!(hops[0].peers, [LPC]);
}

#[test]
fn a_bus_that_loops_has_no_path() {
    let looped = vec![super::acs_tree::Func {
        at: (7, 0, 0),
        buses: Some((7, 7)),
        conventional: false,
        alias: None,
    }];
    assert!(path((7, 0, 0), &looped).is_none());
}
