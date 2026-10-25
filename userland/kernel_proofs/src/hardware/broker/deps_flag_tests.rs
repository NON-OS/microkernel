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

//! A platform device whose _DEP the capsule could not satisfy travels with
//! the record's flag, and the table's record carries it; no other flag
//! bit is accepted.

use super::acpi::platform_decode::decode;
use super::acpi::platform_encode::encode;
use super::acpi::platform_record::record;
use super::acpi::platform_wire::{Platform, DEPS_UNMET, GPIO};
use super::device::DEVICE_FLAG_DEPS_UNMET;
use super::round_trip_tests::platform;

#[test]
fn an_unmet_dependency_travels_as_a_flag() {
    let flagged = Platform {
        flags: DEPS_UNMET,
        ..platform(GPIO, b"INT34C5", &[(0x1000, 0x1000)], b"\\_SB.GPI0")
    };
    let b = encode(&flagged).expect("encodes");
    assert_eq!(decode(&b), Some(flagged.clone()));
    assert_eq!(record(&flagged, 0).flags & DEVICE_FLAG_DEPS_UNMET, DEVICE_FLAG_DEPS_UNMET);
    let plain = Platform { flags: 0, ..flagged.clone() };
    assert_eq!(record(&plain, 0).flags & DEVICE_FLAG_DEPS_UNMET, 0);
    assert_eq!(encode(&Platform { flags: 2, ..flagged }), None, "an unknown flag");
    let mut raw = b;
    raw[4] = 0x80;
    assert_eq!(decode(&raw), None);
}
