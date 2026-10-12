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

//! Which firmware tables the ACPI capsule may read. Pure: `kernel_proofs`
//! mounts this file as it is.
//!
//! Only what running AML needs: the definition blocks, the FADT and FACS for
//! the fixed hardware and the global lock, and the ECDT for the embedded
//! controller before its _REG. Tables that name the machine (MSDM carries an
//! operating system licence key, SLIC an OEM certificate) or the boot's
//! measurements stay with the kernel.

/// ACPI 6.5 tables 5.5 and 5.6: the definition blocks and the fixed tables
/// AML depends on.
const ALLOWED: [[u8; 4]; 6] = [*b"DSDT", *b"SSDT", *b"PSDT", *b"FACP", *b"FACS", *b"ECDT"];

/// "OEMx" tables are reserved for OEM definition blocks a LoadTable may name
/// (table 5.6); only their first three bytes are fixed.
pub(super) fn readable(sig: [u8; 4]) -> bool {
    ALLOWED.contains(&sig) || sig[..3] == *b"OEM"
}
