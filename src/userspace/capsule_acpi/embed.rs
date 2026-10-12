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

//! The ACPI capsule's signed bytes, built by `nonos-mk` (userland/capsule_acpi/
//! Capsule.mk) and sealed with the standard capsules. Only a build with
//! `nonos-capsule-acpi` declares this module.

pub(super) const ACPI_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_acpi/target/",
    env!("NONOS_USER_TARGET"),
    "/release/acpi"
));

pub(super) const ACPI_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/acpi.nonos_id_cert.bin");

pub(super) const ACPI_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/acpi.manifest.bin");

pub(super) const ACPI_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/acpi.zk_trailer.bin");
