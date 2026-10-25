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

//! Remapped interrupts on AMD-Vi (`intr_format` says why): a claimed
//! device's MSIs by handle (route.rs), and sources whose messages carry the
//! vector itself, an I/O APIC's lines among them, by vector (by_vector.rs).

mod by_vector;
mod handles;
mod irte_write;
mod route;
pub mod slots;
mod table;

pub use by_vector::{is_remapping, release, route};
pub use handles::is_amd;
pub use route::{release_msi, route_msi};
