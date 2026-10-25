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

//! The handoff flags' names, in bit order (constants.rs flags re-exports it).

pub fn flag_names(flags: u64) -> &'static [&'static str] {
    const NAMES: [&str; 17] = [
        "W^X",
        "NXE",
        "SMEP",
        "SMAP",
        "UMIP",
        "IDMAP",
        "FB",
        "ACPI",
        "TPM",
        "SECBOOT",
        "ZK",
        "INSTALL",
        "HARDENED",
        "SAFE",
        "AIRGAP",
        "RECOVERY",
        "FB-REFUSED",
    ];
    &NAMES[..(64 - flags.leading_zeros() as usize).min(17)]
}
