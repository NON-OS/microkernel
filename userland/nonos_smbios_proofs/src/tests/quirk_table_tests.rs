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

//! The table's rules: the table that ships passes them (it is empty), and
//! an entry without a cited reason, without fields, with an empty pattern,
//! without a device, or reusing a name is caught.

use crate::quirk::*;

const ON: &[DmiMatch] = &[DmiMatch { field: Field::Vendor, is: Match::Exact("LENOVO") }];
const EMPTY: &[DmiMatch] = &[DmiMatch { field: Field::Product, is: Match::Prefix("") }];
const DEV: DeviceMatch = DeviceMatch::Pci { vendor: 0x8086, device: 0x51F0 };

fn q(
    name: &'static str,
    reason: &'static str,
    dmi: &'static [DmiMatch],
    device: DeviceMatch,
) -> Quirk {
    Quirk { name, reason, dmi, device, value: 1 }
}

#[test]
fn the_shipping_table_passes_and_is_empty() {
    assert_eq!(check(QUIRKS), Ok(()));
    assert!(QUIRKS.is_empty(), "entries come with the unit that needs them");
}

#[test]
fn every_rule_is_checked() {
    let good = q("a", "erratum 12", ON, DEV);
    assert_eq!(check(&[good]), Ok(()));
    assert_eq!(check(&[q("b", " ", ON, DEV)]), Err(("b", Problem::NoReason)));
    assert_eq!(check(&[q("c", "x", &[], DEV)]), Err(("c", Problem::NoFields)));
    assert_eq!(check(&[q("d", "x", EMPTY, DEV)]), Err(("d", Problem::EmptyPattern)));
    let no_hid = q("e", "x", ON, DeviceMatch::Acpi { hid: "" });
    assert_eq!(check(&[no_hid]), Err(("e", Problem::NoDevice)));
    assert_eq!(check(&[good, q("a", "y", ON, DEV)]), Err(("a", Problem::DuplicateName)));
}
