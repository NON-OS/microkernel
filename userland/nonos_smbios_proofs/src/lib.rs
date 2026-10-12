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

//! Proofs for `nonos_smbios`, built on the host against the crate's own
//! sources, included by path.

extern crate alloc;

#[path = "../../nonos_smbios/src/chassis.rs"]
pub mod chassis;
#[path = "../../nonos_smbios/src/entry.rs"]
pub mod entry;
#[path = "../../nonos_smbios/src/identity.rs"]
pub mod identity;
#[path = "../../nonos_smbios/src/quirk/mod.rs"]
pub mod quirk;
#[path = "../../nonos_smbios/src/structure.rs"]
pub mod structure;
#[path = "../../nonos_smbios/src/text.rs"]
pub mod text;
#[path = "../../nonos_smbios/src/walk.rs"]
pub mod walk;

#[cfg(test)]
mod build;
#[cfg(test)]
mod tests;
