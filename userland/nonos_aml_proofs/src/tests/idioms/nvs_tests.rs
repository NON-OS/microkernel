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

//! A global NVS area (the pattern of ACPI 6.5, 5.5.2.4's OperationRegion
//! and Field examples, at the scale firmware uses it): one SystemMemory
//! region and dozens of fields of mixed widths, every one read and summed,
//! one written back, each access inside the region at an AnyAcc width.

use super::nvs_rig::{nvs, BASE, LEN};
use super::run::ok;

#[test]
fn every_field_read_inside_the_region() {
    let (mut r, expected) = nvs();
    let (v, ran) = ok(&mut r, "\\SUMN");
    assert_eq!(v, expected);
    for (space, addr, width, _) in &r.probe.log {
        assert_eq!(*space, 0);
        assert!(*addr >= BASE && addr + u64::from(*width) <= BASE + LEN, "{addr:#x}+{width}");
        assert!(matches!(width, 1 | 2 | 4 | 8));
    }
    assert!(ran.steps < 1000, "{} steps for 48 fields", ran.steps);
}

#[test]
fn a_field_written_back_reads_back() {
    let (mut r, _) = nvs();
    assert_eq!(ok(&mut r, "\\WOSY").0, 0x7DD);
    assert_eq!((r.probe.bytes[&(0, BASE + 1)], r.probe.bytes[&(0, BASE + 2)]), (0xDD, 0x07));
}
