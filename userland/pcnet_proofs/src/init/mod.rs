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

//! The driver's bring-up files, each named so the tests can drive the whole
//! sequence and the pure pieces on their own.

#[path = "../../../capsule_driver_pcnet/src/init/chip.rs"]
pub mod chip;
#[path = "../../../capsule_driver_pcnet/src/init/finish.rs"]
mod finish;
#[path = "../../../capsule_driver_pcnet/src/init/hold.rs"]
mod hold;
#[path = "../../../capsule_driver_pcnet/src/init/init_block.rs"]
pub mod init_block;
#[path = "../../../capsule_driver_pcnet/src/init/reset.rs"]
mod reset;
#[path = "../../../capsule_driver_pcnet/src/init/run.rs"]
mod run;
#[path = "../../../capsule_driver_pcnet/src/init/start.rs"]
mod start;
#[path = "../../../capsule_driver_pcnet/src/init/station_address.rs"]
mod station_address;

pub use finish::finish;
