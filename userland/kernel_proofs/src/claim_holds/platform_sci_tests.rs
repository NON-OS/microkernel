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

//! The SCI's record, which the ACPI capsule claims to bind the interrupt,
//! held for that capsule alone (`acpi::sci`).

use super::hold_platform::*;
use super::hold_rule::Hold;

const ACPI: u8 = BUS_ACPI;

#[test]
fn the_sci_is_held_for_the_acpi_capsule_and_no_other_record_of_its_class() {
    assert_eq!(hold_for_platform(ACPI, OTHER, 0, PNP, PNP_ACPI), Some(Hold::ForService("acpi")));
    assert_eq!(ACPI_SERVICE, "acpi");
    for (vendor, device) in [(PNP, PS2_KBD), (0x0002, PNP_ACPI), (0, 0)] {
        let hold = hold_for_platform(ACPI, OTHER, 0, vendor, device);
        assert_eq!(hold, Some(Hold::Refused), "{vendor:#x}:{device:#x}");
    }
    assert_eq!(hold_for_platform(ACPI, INPUT, 0, PNP, PNP_ACPI), Some(Hold::Refused));
}
