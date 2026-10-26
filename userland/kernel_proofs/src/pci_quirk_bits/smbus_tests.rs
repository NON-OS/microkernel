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

//! The SMBus host's HOSTC bits, and no more.

use super::quirk_bits::{only, writable, Ident, Route};

const SMBUS: Ident = Ident {
    vendor: 0x8086,
    class: 0x0C,
    subclass: 0x05,
    prog_if: 0,
    pcie_cap: None,
    before: [0xFF; 3],
    device: 0,
    route: Route::NONE,
};

#[test]
fn the_intel_smbus_host_gets_host_enable_and_i2c_mode() {
    assert_eq!(writable(&SMBUS, 0x40), Some(0x0005));
    // SMI routing (bit 1), the soft reset (bit 3) and the byte above stay.
    assert!(only(0x0005, 0x0003, 0x0006), "host on, I2C mode off");
    assert!(!only(0x0005, 0x0001, 0x0003), "SMI routing off");
    assert!(!only(0x0005, 0x0109, 0x0001), "the next byte");
}

#[test]
fn nothing_else_of_the_smbus_host_and_no_other_host_is_writable() {
    for offset in [0x00, 0x04, 0x10, 0x20, 0x3C, 0x41, 0x42, 0x44] {
        assert_eq!(writable(&SMBUS, offset), None, "{offset:#x}");
    }
    let amd = Ident { vendor: 0x1022, ..SMBUS };
    assert_eq!(writable(&amd, 0x40), None, "an AMD FCH SMBus has no HOSTC");
    let lpss = Ident { subclass: 0x80, ..SMBUS };
    assert_eq!(writable(&lpss, 0x40), None, "an LPSS I2C controller");
}
