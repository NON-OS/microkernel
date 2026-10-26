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

//! Which architectures are ACPI platforms, and which bind PCI message
//! interrupts, said here and nowhere else. `acpi_only!` declares a module, or a
//! `use`, that exists only where firmware describes the machine through ACPI
//! (the ACPI broker and capsule, their writers into broker state); `msi_only!`
//! one that exists only where a capsule's device may raise MSI or MSI-X (the
//! broker's MSI side table). `else` declares the counterpart everywhere else.
//! Only `mod` and `use` items go inside, never code: rustfmt does not format a
//! macro's body (scripts/check_platform_gates.py).

macro_rules! acpi_only {
    (else $($item:item)*) => { $( #[cfg(not(target_arch = "x86_64"))] $item )* };
    ($($item:item)*) => { $( #[cfg(target_arch = "x86_64")] $item )* };
}
pub(crate) use acpi_only;

macro_rules! msi_only {
    (else $($item:item)*) => { $( #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))] $item )* };
    ($($item:item)*) => { $( #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))] $item )* };
}
pub(crate) use msi_only;
