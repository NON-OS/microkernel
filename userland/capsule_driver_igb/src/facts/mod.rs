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

//! The fact sheet: register offsets, bits, part traits, ids and bounds,
//! one module per block. Intel's datasheets could not be opened where this
//! was written; every value says where it was seen and is unverified until
//! someone checks it against the 82575, 82576, 82580, I350 and I210
//! datasheets.

pub mod bounds;
pub mod ids;
pub mod mac;
pub mod parts;
pub mod phy;
pub mod regs;
pub mod rx;
pub mod tx;
