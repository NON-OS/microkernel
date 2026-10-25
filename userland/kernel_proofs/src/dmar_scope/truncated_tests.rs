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

//! The boot line for a truncated unit names a count, never a requester id.

use super::dmar_scope_count::pci_scopes_named;
use super::tests::{drhd, scope};
use super::truncated::{line, LINE_MAX};

fn said(unit: usize, named: u16) -> String {
    let mut out = [0u8; LINE_MAX];
    let n = line(unit, named, 255, &mut out);
    String::from_utf8(out[..n].to_vec()).unwrap()
}

#[test]
fn past_the_cap_it_names_the_count_and_the_refusal() {
    assert_eq!(
        said(0, 300),
        "[DMAR] remapping unit 0 names 300 device scopes; NONOS keeps 255; devices past that are refused"
    );
}

#[test]
fn a_scope_that_did_not_parse_is_said_as_such() {
    assert_eq!(
        said(3, 2),
        "[DMAR] remapping unit 3: a device scope could not be read; the devices it names are refused"
    );
    assert!(!said(7, 255).contains("names 255"), "at the cap is not past it");
}

#[test]
fn the_count_takes_pci_scopes_only_and_stops_at_a_malformed_one() {
    let mut s =
        vec![scope(3, 0xf0, &[(0x1f, 0)]), scope(1, 0, &[(2, 0)]), scope(2, 0, &[(0x1c, 0)])];
    assert_eq!(pci_scopes_named(&drhd(0, 0, &s), 16), 2, "an IOAPIC scope names no PCI device");
    s.push(vec![1, 3, 0]);
    s.push(scope(1, 0, &[(3, 0)]));
    assert_eq!(pci_scopes_named(&drhd(0, 0, &s), 16), 2, "nothing after a malformed entry");
}
