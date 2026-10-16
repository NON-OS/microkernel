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

//! The numbers `hold_platform` keys by, held to the kernel's own at build
//! time: the pure rule cannot name them, and a class id or PNP identity
//! renumbered on one side alone would open a record the rule meant to hold.

use super::hold_platform as p;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::platform as ps2;
use crate::hardware::broker::BusKind;

const _: () = assert!(p::BUS_ACPI == BusKind::Acpi as u8);
const _: () = assert!(p::SERIAL == ids::SERIAL && p::INPUT == ids::INPUT);
const _: () = assert!(p::GPIO_CTRL == ids::GPIO_CTRL);
const _: () = assert!(p::PNP == ps2::PNP_VENDOR_PS2_KBD && p::PNP == ps2::PNP_VENDOR_PS2_AUX);
const _: () = assert!(p::PS2_KBD == ps2::PNP_DEVICE_PS2_KBD);
const _: () = assert!(p::PS2_AUX == ps2::PNP_DEVICE_PS2_AUX);
