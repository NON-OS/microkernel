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

//! How a driver waits for its device: spin briefly, then block on the
//! device interrupt with a ceiling, else yield and then sleep. One policy
//! for every capsule, taken from the xHCI driver, rather than a copy each.

mod park;
mod step;

pub use park::IrqPark;
pub use step::{step, Step, PARK_MS, SPIN_BUDGET, YIELD_BUDGET};
