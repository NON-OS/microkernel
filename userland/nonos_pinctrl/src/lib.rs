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

//! Where the level of a firmware-named GPIO pin is read.
//!
//! An ACPI GpioInt descriptor names a controller and a pin number in the
//! firmware's numbering. On the AMD FCH bank the number is the pin itself,
//! one dword each. On an Intel PCH the number has to be placed among pad
//! groups and community windows by a layout; with no layout derived from a
//! published datasheet (`layouts`), an Intel pin is refused. Nothing here
//! touches a register: the caller maps the window and reads what this
//! names. Register facts are recalled from Intel's and AMD's published
//! documents, which could not be read here, and are marked unverified.

#![no_std]

mod amd_regs;
mod controller;
mod error;
mod group;
mod intel_regs;
mod layouts;
mod locate;
mod resolve;

pub use amd_regs::{amd_alive, amd_pin_reg, AMD_PIN_STS};
pub use controller::{controller, Controller};
pub use error::{PadError, PinError};
pub use group::{Community, Group, Layout};
pub use intel_regs::{intel_pad_stride, intel_padcfg0, INTEL_PADBAR, INTEL_REVID, INTEL_RXSTATE};
pub use layouts::LAYOUTS;
pub use locate::{locate, Line};
pub use resolve::{resolve, Pad};
