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

// Shared with HID over I2C, whose reset recovery lets a held click go
// (button_held); no reset here leaves one held.
#[allow(dead_code)]
#[path = "../../../capsule_driver_i2c_hid/src/input/gesture/mod.rs"]
pub mod gesture;
pub mod parse_report;
mod poll;
#[path = "../../../capsule_driver_i2c_hid/src/input/post.rs"]
mod post;
#[path = "../../../capsule_driver_i2c_hid/src/input/publish.rs"]
mod publish;
#[path = "../../../capsule_driver_i2c_hid/src/input/publish_screen.rs"]
mod publish_screen;
#[path = "../../../capsule_driver_i2c_hid/src/input/publish_touch.rs"]
mod publish_touch;
#[path = "../../../capsule_driver_i2c_hid/src/input/sample.rs"]
pub mod sample;

pub use gesture::TouchGesture;
pub use poll::poll;
