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

//! The platform's own services: ACPI, attestation, the boot splash, power
//! and the policy store.

#[cfg(feature = "nonos-capsule-acpi")]
#[path = "capsule_acpi/mod.rs"]
pub mod capsule_acpi;
#[path = "capsule_attest/mod.rs"]
pub mod capsule_attest;
#[path = "capsule_boot_splash/mod.rs"]
pub mod capsule_boot_splash;
#[path = "capsule_power/mod.rs"]
pub mod capsule_power;
#[path = "capsule_policy/mod.rs"]
pub mod capsule_policy;
