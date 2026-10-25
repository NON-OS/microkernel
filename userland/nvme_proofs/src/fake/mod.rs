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

//! The controller the tests play: a register block in host memory, the
//! queues the driver maps there, and a controller that runs on each clock
//! read the driver makes.

mod answer;
mod armed;
mod controller;
mod fresh;
mod io;
mod registers;
mod ring;
mod seen;
mod serve;
mod shutdown;
mod stall;
mod up;

pub(crate) use armed::{left_as, left_running, LEFT};
pub(crate) use controller::Fake;
pub(crate) use registers::{info, install, register, registers};
pub(crate) use seen::disables;
pub(crate) use up::{up, up_on, Up};
