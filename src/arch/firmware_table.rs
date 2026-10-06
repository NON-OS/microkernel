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

//! The physical address of an ACPI table by signature. ACPI is read on x86_64;
//! an aarch64 or riscv64 board describes itself in its device tree, so no ACPI
//! table is found there.

#[inline]
pub fn acpi_table_address(signature: &[u8; 4]) -> Option<u64> {
    #[cfg(target_arch = "x86_64")]
    {
        crate::arch::x86_64::acpi::table_address(signature)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = signature;
        None
    }
}
