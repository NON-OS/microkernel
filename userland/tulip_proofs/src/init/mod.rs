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

#[path = "../../../capsule_driver_tulip/src/init/finish.rs"]
mod finish;
#[path = "../../../capsule_driver_tulip/src/init/hold.rs"]
mod hold;
#[path = "../../../capsule_driver_tulip/src/init/reset.rs"]
pub mod reset;
#[path = "../../../capsule_driver_tulip/src/init/run.rs"]
mod run;
#[path = "../../../capsule_driver_tulip/src/init/setup_frame.rs"]
pub mod setup_frame;
#[path = "../../../capsule_driver_tulip/src/init/start.rs"]
mod start;
#[path = "../../../capsule_driver_tulip/src/init/station_address.rs"]
mod station_address;

pub use finish::finish;
pub use reset::running;
