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

//! The inventory names a network, storage or SMBus part as the capsule that
//! takes it does: the kernel's tables and classification included by path,
//! beside the capsules' own.

#[path = "../../../../src/hardware/inventory/broadcom_ids.rs"]
mod broadcom_ids;
#[path = "../../../../src/hardware/inventory/classify_network.rs"]
mod classify_network;
#[path = "../../../../src/hardware/inventory/classify_serial_bus.rs"]
mod classify_serial_bus;
#[path = "../../../../src/hardware/inventory/classify_storage.rs"]
mod classify_storage;
#[path = "../../../../src/hardware/inventory/family.rs"]
#[allow(dead_code)]
mod family;
#[path = "../../../../src/hardware/inventory/intel_ethernet_ids.rs"]
mod intel_ethernet_ids;
#[path = "../../../../src/hardware/inventory/realtek_ids.rs"]
mod realtek_ids;
// Mounted once, by the VMD proofs; classify_storage reaches it here.
use crate::vmd_domain::inventory_vmd as vmd;

#[path = "../../../capsule_driver_e1000/src/constants/pci.rs"]
#[allow(dead_code)]
mod e1000;
#[path = "../../../capsule_driver_e1000e/src/constants/ids.rs"]
#[allow(dead_code)]
mod e1000e;
#[allow(dead_code)]
mod igb;
#[path = "../../../capsule_driver_igc/src/constants/pci.rs"]
#[allow(dead_code)]
mod igc;
#[allow(dead_code)]
mod ixgbe;
#[path = "../../../capsule_driver_rtl8139/src/constants/pci.rs"]
#[allow(dead_code)]
mod rtl8139;
#[path = "../../../capsule_driver_rtl8169/src/constants/pci.rs"]
#[allow(dead_code)]
mod rtl8169;
#[path = "../../../capsule_driver_tg3/src/constants/pci.rs"]
#[allow(dead_code)]
mod tg3;

mod broadcom_tests;
mod classify_tests;
mod serial_bus_tests;
mod table_tests;
