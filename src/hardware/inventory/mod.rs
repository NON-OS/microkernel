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
mod classify_device;
mod classify_display;
mod classify_network;
mod classify_serial_bus;
mod classify_storage;
mod driver;
mod emmc;
mod family;
mod intel_ethernet_ids;
mod missing;
mod platform_sd;
mod present;
mod realtek_ids;
mod record;
mod scan;
mod sd_host;
mod state;
mod support;
mod vmd;

pub use classify::classify_family;
pub use classify_device::classify_device;
pub use driver::family_driver;
pub use emmc::{is_intel_emmc, INTEL_EMMC_DEVICE_IDS};
pub use family::HardwareFamily;
pub use intel_ethernet_ids::{
    E1000E_DEVICE_IDS, E1000_DEVICE_IDS, IGB_DEVICE_IDS, IGC_DEVICE_IDS, IXGBE_DEVICE_IDS,
};
pub use missing::missing_path;
pub use present::present;
pub use realtek_ids::{RTL8139_DEVICE_ID, RTL8169_DEVICE_IDS, RTL8821CE_DEVICE_ID};
pub use record::InventoryRecord;
pub use scan::scan;
pub use sd_host::{is_sd_host, INTEL_SDIO_DEVICE_IDS};
pub use state::SupportState;
pub use support::support_state;
pub use vmd::{is_intel_vmd, INTEL_VMD_DEVICE_IDS};
