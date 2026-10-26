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

/*
 * The broker's device record, BARs and flags, each file by path as the
 * kernel builds it; the kernel's `device.rs` mounts the same four.
 */

#[path = "../../../../../../src/hardware/broker/device/bar.rs"]
mod bar;
#[path = "../../../../../../src/hardware/broker/device/bus.rs"]
mod bus;
#[path = "../../../../../../src/hardware/broker/device/flags.rs"]
mod flags;
#[cfg(test)]
mod libc_tests;
#[cfg(test)]
pub mod libc_types;
#[path = "../../../../../../src/hardware/broker/device/record.rs"]
mod record;

pub(super) use bar::BAR_KIND_MMIO;
pub use bar::{Bar, BarKind};
pub use bus::BusKind;
pub use flags::{
    BAR_FLAG_FIXED, DEVICE_FLAG_DEPS_UNMET, DEVICE_FLAG_IRQ_ACTIVE_LOW, DEVICE_FLAG_IRQ_EDGE,
};
pub use record::DeviceRecord;
