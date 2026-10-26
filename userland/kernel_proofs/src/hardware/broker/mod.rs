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

// The broker's records, class table (its code map checked whole, L79), BAR choice, external-facing
// ports, firmware's reserved ranges and the device tree's DesignWare I2C stop routine, by path.

pub mod acpi;
#[path = "../../../../../src/hardware/broker/class.rs"]
pub mod class;
pub mod device;
#[path = "../../../../../src/hardware/broker/external_range.rs"]
pub mod external_range;
#[cfg(test)]
mod fdt_i2c;
#[path = "../../../../../src/hardware/broker/irq/gone.rs"]
pub mod irq_gone;
pub mod mmio;
pub mod reserved;
pub mod table;

pub use device::{Bar, BarKind, DeviceRecord, BAR_FLAG_FIXED};

#[cfg(test)]
mod bars_tests;
#[cfg(test)]
mod button_tests;
#[cfg(test)]
mod class_tests;
#[cfg(test)]
mod deps_flag_tests;
#[cfg(test)]
mod dgon_tests;
#[cfg(test)]
mod external_tests;
#[cfg(test)]
mod first_pass_tests;
#[cfg(test)]
mod irq_flag_tests;
#[cfg(test)]
mod loaded_tests;
#[cfg(test)]
mod numa_tests;
#[cfg(test)]
mod osc_grant_tests;
#[cfg(test)]
mod restart_tests;
#[cfg(test)]
mod round_trip_tests;
#[cfg(test)]
mod same_group_tests;
#[cfg(test)]
mod slot_port_tests;
#[cfg(test)]
mod slot_tests;
#[cfg(test)]
mod table_measure_tests;
#[cfg(test)]
mod thermal_limit_tests;
#[cfg(test)]
mod usb_port_tests;
