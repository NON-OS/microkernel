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
mod acpi_touchpad;
mod classify;
mod defs;
mod find_controllers;
mod find_touchpad_addrs;

pub use acpi_touchpad::{AcpiTouchpad, MAX_TARGETS};
pub use defs::{Found, CLASS_GPIO_CTRL, MAX_CONTROLLERS};
pub use find_controllers::find_controllers;
pub use find_touchpad_addrs::{find_touchpad_addrs, touchpad_of};
