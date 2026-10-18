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

//! Arm's IO Remapping Table (DEN0049) read into the records the broker
//! already takes from a device tree, for machines that describe themselves
//! with ACPI. Pure: it reads a table it is handed and maps nothing.

mod compose;
mod convert;
mod error;
mod layout;
mod mapping;
mod node;
mod read;
mod records;
mod route;
mod through;
mod units;

pub use error::IortError;
pub use records::{root_complexes, RootComplex, MAX_ROOT_COMPLEXES};
