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

mod classify;
mod classify_display;
mod classify_network;
mod classify_serial_bus;
mod classify_storage;
mod driver;
mod emmc;
mod family;
mod missing;
mod present;
mod record;
mod scan;
mod state;
mod support;
mod vmd;

pub use classify::{classify_device, classify_family};
pub use driver::family_driver;
pub use emmc::{is_intel_emmc, INTEL_EMMC_DEVICE_IDS};
pub use family::HardwareFamily;
pub use missing::missing_path;
pub use present::present;
pub use record::InventoryRecord;
pub use scan::scan;
pub use state::SupportState;
pub use support::support_state;
pub use vmd::{is_intel_vmd, INTEL_VMD_DEVICE_IDS};
