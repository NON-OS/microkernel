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

//! The driver's bring-up files, so the tests can drive the whole sequence.

#[path = "../../../capsule_driver_vmxnet3/src/init/activate.rs"]
mod activate;
#[path = "../../../capsule_driver_vmxnet3/src/init/finish.rs"]
mod finish;
#[path = "../../../capsule_driver_vmxnet3/src/init/run.rs"]
mod run;
#[path = "../../../capsule_driver_vmxnet3/src/init/shared_area.rs"]
mod shared_area;
#[path = "../../../capsule_driver_vmxnet3/src/init/station_address.rs"]
mod station_address;
#[path = "../../../capsule_driver_vmxnet3/src/init/version.rs"]
mod version;

pub use finish::finish;
