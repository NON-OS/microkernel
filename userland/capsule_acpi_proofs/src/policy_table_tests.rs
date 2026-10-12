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

//! The per-op table as the maintainers set it, op by op.

use crate::policy::{allowed, Allowed};

#[test]
fn the_table_the_lead_set() {
    let names = |op| match allowed(op) {
        Allowed::Services(n) => n.to_vec(),
        _ => Vec::new(),
    };
    let (power, shell): (&[u8], &[u8]) = (b"power", b"desktop_shell");
    for op in [3, 4, 6, 7, 14, 17] {
        assert_eq!(names(op), [power, shell], "op {op}");
    }
    assert_eq!(names(5), [power], "PREPARE_SLEEP");
    assert_eq!(names(15), [power], "TIME_SET");
    assert_eq!(names(8), [shell], "SMBIOS");
    assert_eq!(names(18), [shell], "TPM_PPI");
    assert_eq!(names(9), [&b"driver.i2c_pci0"[..], b"driver.i2c_hid0"], "SERIAL");
    assert_eq!(allowed(10), Allowed::AnyAdmitted, "DEVICE_POWER: the kernel's holder test");
    assert_eq!(names(11), [&b"driver.ps2_kbd0"[..]], "LEGACY");
    assert_eq!(allowed(12), Allowed::AnyAdmitted, "PROPERTIES: the kernel's holder test");
    assert_eq!(allowed(13), Allowed::AnyAdmitted, "GPIO_EVENT: the kernel's holder test");
    assert_eq!(allowed(16), Allowed::AnyAdmitted, "QUIRKS: the kernel's holder test");
    for op in [0, 1, 2, 19, 255] {
        assert_eq!(allowed(op), Allowed::Nobody, "op {op}");
    }
}
