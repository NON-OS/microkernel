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

//! BGRT's parser, the BMP check and the logo copy, through the aarch64
//! tables' mount of the one copy the x86 kernel also builds.

pub use crate::arm_acpi::bgrt::{bgrt_entry, bmp_check};

#[cfg(test)]
mod bmp_tests;
#[cfg(test)]
mod copy_tests;
#[cfg(test)]
mod tests;
