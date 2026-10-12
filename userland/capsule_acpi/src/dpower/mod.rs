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

//! Device power for the capsules that drive the devices: their D-state,
//! read and moved through _PSx and the power resources (ACPI 6.5, ch. 7).

mod find;
mod hidden;
mod moves;
mod settle;
mod wire;

pub use hidden::reveal_hidden;
pub use settle::{tick, Hidden};
pub use wire::device_power;
