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

//! The ranges firmware keeps for itself: the I/O and memory a motherboard
//! resource device (PNP0C01, PNP0C02) names in its _CRS, which no driver
//! may be handed (ACPI 6.5, 6.4 for the descriptors).

mod find;
mod fixed;
mod space;

pub use find::{motherboard_ranges, MOTHERBOARD_IDS};
pub use fixed::{reserved, Reserved, Space};
