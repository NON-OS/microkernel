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

//! Quirks matched on a machine's identity and the device its driver holds:
//! every field must match, exactly or by prefix; a field firmware left
//! blank matches nothing; the device must be the one the quirk names.

use crate::identity::Identity;
use crate::quirk::*;

fn laptop() -> Identity {
    Identity {
        vendor: "Framework".into(),
        product: "Laptop 13 (AMD Ryzen 7040Series)".into(),
        version: "A5".into(),
        bios_vendor: "INSYDE Corp.".into(),
        bios_version: "03.05".into(),
        board: String::new(),
        ..Identity::default()
    }
}

const ON: &[DmiMatch] = &[
    DmiMatch { field: Field::Vendor, is: Match::Exact("Framework") },
    DmiMatch { field: Field::Product, is: Match::Prefix("Laptop 13 (AMD") },
];
const TOUCHPAD: DeviceMatch = DeviceMatch::Acpi { hid: "PIXA3854" };

fn quirk(name: &'static str, dmi: &'static [DmiMatch], device: DeviceMatch) -> Quirk {
    Quirk { name, reason: "test", dmi, device, value: 7 }
}

#[test]
fn every_field_must_match() {
    let id = laptop();
    assert!(dmi_matches(&id, ON), "exact vendor, prefix product");
    let wrong: &[DmiMatch] = &[DmiMatch { field: Field::Vendor, is: Match::Exact("Framewor") }];
    assert!(!dmi_matches(&id, wrong), "exact means all of it");
    let prefix: &[DmiMatch] = &[DmiMatch { field: Field::BiosVersion, is: Match::Prefix("03.") }];
    assert!(dmi_matches(&id, prefix));
    let one_off: &[DmiMatch] = &[ON[0], DmiMatch { field: Field::Version, is: Match::Exact("A6") }];
    assert!(!dmi_matches(&id, one_off), "one field off is no match");
}

#[test]
fn blank_and_empty_match_nothing() {
    let id = laptop();
    let blank: &[DmiMatch] = &[DmiMatch { field: Field::Board, is: Match::Prefix("") }];
    assert!(!dmi_matches(&id, blank), "a blank field, an empty pattern");
    let board: &[DmiMatch] = &[DmiMatch { field: Field::Board, is: Match::Exact("FRANMDCP07") }];
    assert!(!dmi_matches(&id, board), "firmware left the board blank");
    assert!(!dmi_matches(&id, &[]), "no fields fires nowhere");
    assert!(!dmi_matches(&Identity::default(), ON), "a machine that says nothing");
}

#[test]
fn the_device_must_be_the_one_named() {
    let id = laptop();
    let table = [
        quirk("touchpad_debounce", ON, TOUCHPAD),
        quirk("wifi_aspm_off", ON, DeviceMatch::Pci { vendor: 0x14C3, device: 0x0616 }),
    ];
    let names =
        |d: Device<'_>| -> Vec<&str> { lookup(&table, &id, &d).iter().map(|q| q.name).collect() };
    assert_eq!(names(Device::Acpi { hid: b"PIXA3854" }), ["touchpad_debounce"]);
    assert_eq!(names(Device::Pci { vendor: 0x14C3, device: 0x0616 }), ["wifi_aspm_off"]);
    assert!(names(Device::Pci { vendor: 0x14C3, device: 0x0617 }).is_empty());
    assert!(names(Device::Acpi { hid: b"PIXA3855" }).is_empty());
    assert!(lookup(&table, &Identity::default(), &Device::Acpi { hid: b"PIXA3854" }).is_empty());
}
