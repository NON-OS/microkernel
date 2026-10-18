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

//! GPIO-signalled ACPI events (ACPI 6.5, 5.6.5): the pins a GPIO
//! controller's _AEI lists as event sources, and the method each runs when
//! the controller's driver reports its interrupt.

mod pins;
mod signal;

pub use pins::{aei_pins, AeiPin, MAX_PINS};
pub use signal::{method_for, signal, AeiError, Ran};
