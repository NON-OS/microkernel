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
//! firmware's numbering. On Intel PCH pinctrl the number is a GPIO base in
//! a pad group (Linux pinctrl-intel.c intel_gpio_to_pin), so the pad and the
//! community window it sits in come from a per-platform table; on AMD the
//! number is the pin itself (Linux pinctrl-amd.c). Nothing here touches a
//! register: the caller maps the window and reads what this names.

#![no_std]

mod amd_regs;
mod controller;
mod error;
mod group;
mod intel_regs;
mod locate;
mod resolve;
mod tables;

pub use amd_regs::{amd_alive, amd_pin_reg, AMD_PIN_STS};
pub use controller::{controller, Controller};
pub use error::{PadError, PinError};
pub use group::{gpp, Community, Group, Layout, MATCH, NOMAP, ZERO};
pub use intel_regs::{intel_pad_stride, intel_padcfg0, INTEL_PADBAR, INTEL_REVID, INTEL_RXSTATE};
pub use locate::{locate, Line};
pub use resolve::{resolve, Pad};
pub use tables::LAYOUTS;
