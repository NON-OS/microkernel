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

//! Only the tables running AML needs reach the capsule.

use super::table_allow::readable;

#[test]
fn only_the_tables_aml_needs_are_readable() {
    for sig in [b"DSDT", b"SSDT", b"PSDT", b"FACP", b"FACS", b"ECDT", b"OEM1", b"OEMB"] {
        assert!(readable(*sig), "{}", String::from_utf8_lossy(sig));
    }
    for sig in [b"MSDM", b"SLIC", b"TPM2", b"TCPA", b"APIC", b"DMAR", b"XSDT", b"RSDT", b"OEZ1"] {
        assert!(!readable(*sig), "{}", String::from_utf8_lossy(sig));
    }
}
