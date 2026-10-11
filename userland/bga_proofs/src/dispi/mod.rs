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

//! The DISPI files, from the shipping tree, named as the capsule names them.

#[path = "../../../capsule_driver_bga/src/dispi/choose.rs"]
mod choose;
#[path = "../../../capsule_driver_bga/src/dispi/clear.rs"]
mod clear;
#[path = "../../../capsule_driver_bga/src/dispi/dispi_off.rs"]
mod dispi_off;
#[path = "../../../capsule_driver_bga/src/dispi/endian.rs"]
mod endian;
#[path = "../../../capsule_driver_bga/src/dispi/mode.rs"]
mod mode;
#[path = "../../../capsule_driver_bga/src/dispi/read.rs"]
mod read;
#[path = "../../../capsule_driver_bga/src/dispi/set_mode.rs"]
mod set_mode;

pub use choose::{choose, Seen};
pub use clear::clear;
pub use dispi_off::dispi_off;
pub use endian::little_endian;
pub use mode::{Choice, Mode};
pub use read::{id, seen, vram};
pub use set_mode::set_mode;
