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

//! What ACS redirects, and the group that is left (K11).

use super::acs_path::{path, Hop};
use super::acs_reach::group_by_hop;
use super::acs_rule::{enabling, redirects, CR, FUNCTION, PORT, RR, SV, UF};
use super::acs_tree::{Bdf, Func};
use super::machine::*;

const ON: Option<(u16, u16)> = Some((PORT, PORT));

fn group(hops: Vec<Hop>, acs: impl FnMut(Bdf, u16) -> Option<(u16, u16)>) -> Vec<Bdf> {
    group_by_hop(hops, acs).into_iter().map(|(member, _)| member).collect()
}

#[test]
fn acs_redirects_with_every_needed_bit_it_offers_on() {
    assert!(redirects(ON, PORT));
    assert!(!redirects(Some((PORT, PORT & !UF)), PORT), "upstream forwarding off");
    assert!(redirects(Some((PORT & !SV, PORT)), PORT), "not offered: hard-wired (K11b)");
    assert!(!redirects(None, PORT), "no ACS at all");
    assert!(redirects(Some((RR | CR, RR | CR)), FUNCTION));
    assert_eq!(enabling(PORT, 0, PORT), PORT, "turned on where offered");
    assert_eq!(enabling(PORT & !UF, 0, PORT), PORT & !UF, "a bit not offered is not written");
}

#[test]
fn an_isolated_endpoint_has_no_group() {
    assert!(group(path(NIC, &machine()).unwrap(), |_, _| ON).is_empty());
}

#[test]
fn a_port_without_acs_puts_its_peers_in_the_group() {
    let no_acs_at = |at: Bdf| move |hop: Bdf, _| if hop == at { None } else { ON };
    let nic = group(path(NIC, &machine()).unwrap(), no_acs_at(DOWN_A));
    assert_eq!(nic, [DOWN_B, WIFI]);
    let wifi = group(path(WIFI, &machine()).unwrap(), no_acs_at(DOWN_B));
    assert_eq!(wifi, [DOWN_A, NIC], "the same group, seen from the other side");
    let sata = group(path(SATA, &machine()).unwrap(), |_, _| None);
    assert_eq!(sata, [LPC], "functions without ACS share a group");
}

#[test]
fn devices_sharing_a_conventional_bus_are_grouped_whatever_acs_says() {
    let mut pci = machine();
    pci.push(Func { at: (2, 3, 0), buses: Some((6, 6)), conventional: true, alias: None });
    pci.extend([(6, 1, 0), (6, 2, 0)].map(|at| Func {
        at,
        buses: None,
        conventional: false,
        alias: None,
    }));
    assert_eq!(group(path((6, 1, 0), &pci).unwrap(), |_, _| ON), [(6, 2, 0)]);
}
