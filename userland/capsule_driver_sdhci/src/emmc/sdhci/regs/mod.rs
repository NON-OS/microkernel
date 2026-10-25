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

//! The fact sheet for the host: SD Host Controller registers and bits,
//! offsets from the slot base. Source: SD Host Controller Simplified
//! Specification 3.00 and 4.20, register chapter, as recalled, unverified.

mod caps;
mod control;
mod interrupt;
mod offsets;
mod transfer;

pub use caps::*;
pub use control::*;
pub use interrupt::*;
pub use offsets::*;
pub use transfer::*;
