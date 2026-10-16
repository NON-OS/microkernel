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

#[path = "../../../../capsule_driver_i2c_hid/src/input/poll/cycle.rs"]
mod cycle;
#[path = "../../../../capsule_driver_i2c_hid/src/input/poll/multitouch.rs"]
mod multitouch;
mod read_frame;
#[path = "../../../../capsule_driver_i2c_hid/src/input/poll/repeat.rs"]
mod repeat;
#[path = "../../../../capsule_driver_i2c_hid/src/input/poll/signal_raw_report.rs"]
mod signal_raw_report;
#[path = "../../../../capsule_driver_i2c_hid/src/input/poll/touch_frame.rs"]
mod touch_frame;

pub use cycle::poll;
