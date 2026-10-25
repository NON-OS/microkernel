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
use super::hold_tree as t;
use super::hold_virt as v;
use crate::hardware::broker::class::ids;
use crate::hardware::broker::described::Interface;
use crate::hardware::broker::platform as ps2;
use crate::hardware::broker::vmbus as vb;
use crate::hardware::broker::{BusKind, VMBUS_DEVICE, VMBUS_VENDOR};

const _: () = assert!(p::BUS_ACPI == BusKind::Acpi as u8);
const _: () = assert!(p::SERIAL == ids::SERIAL && p::INPUT == ids::INPUT);
const _: () = assert!(p::GPIO_CTRL == ids::GPIO_CTRL && p::OTHER == ids::OTHER);
const _: () = assert!(p::BLOCK == ids::BLOCK);
const _: () = assert!(p::PNP == ps2::PNP_VENDOR_PS2_KBD && p::PNP == ps2::PNP_VENDOR_PS2_AUX);
const _: () = assert!(p::PS2_KBD == ps2::PNP_DEVICE_PS2_KBD);
const _: () = assert!(p::PS2_AUX == ps2::PNP_DEVICE_PS2_AUX);
const _: () = assert!(v::BUS_VIRT == BusKind::Virt as u8);
const _: () = assert!(v::VMBUS_VENDOR == VMBUS_VENDOR && v::VMBUS_DEVICE == VMBUS_DEVICE);
const _: () =
    assert!(v::CHANNEL_NETWORK == vb::CHANNEL_NETWORK && v::CHANNEL_SCSI == vb::CHANNEL_SCSI);
const _: () = assert!(v::CHANNEL_IDE == vb::CHANNEL_IDE);
const _: () = assert!(t::BUS_DEVICE_TREE == BusKind::Fdt as u8);
const _: () = assert!(t::TREE_XHCI == Interface::Xhci as u16);
