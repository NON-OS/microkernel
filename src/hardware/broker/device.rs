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

mod bar;
mod bus;
mod flags;
mod record;

pub(super) use bar::BAR_KIND_MMIO;
#[allow(unused_imports)] // the ACPI broker's alone, which only x86 builds
pub(super) use bar::BAR_KIND_PIO;
pub use bar::{Bar, BarKind};
pub use bus::BusKind;
pub use flags::{
    BAR_FLAG_FIXED, DEVICE_FLAG_CLAIMED, DEVICE_FLAG_DEPS_UNMET, DEVICE_FLAG_DISABLED,
    DEVICE_FLAG_DMA_NONCOHERENT, DEVICE_FLAG_EXTERNAL, DEVICE_FLAG_IRQ_ACTIVE_LOW,
    DEVICE_FLAG_IRQ_EDGE,
};
pub(super) use flags::{BAR_FLAG_MEM64, BAR_FLAG_PREFETCH};
pub use record::{DeviceRecord, NUMA_UNKNOWN};
