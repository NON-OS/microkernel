// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

mod classify;
mod classify_device;
mod classify_display;
mod classify_network;
mod classify_serial_bus;
mod classify_storage;
mod driver;
mod emmc;
mod family;
mod i2c_host;
#[cfg(any(feature = "nonos-capsule-driver-i2c-pci", feature = "nonos-capsule-driver-i2c-hid"))]
mod i2c_present;
mod intel_ethernet_ids;
mod mediatek_ids;
mod missing;
mod platform_sd;
mod present;
mod realtek_ids;
mod record;
pub mod report;
mod report_extra;
mod report_hex;
mod report_kinds;
mod report_text;
mod scan;
mod sd_host;
#[cfg(feature = "nonos-capsule-driver-spi")]
mod spi_host;
#[cfg(feature = "nonos-capsule-driver-spi")]
mod spi_present;
mod state;
mod support;
mod vmd;

pub use classify::classify_family;
pub use classify_device::classify_device;
pub use driver::family_driver;
pub use emmc::{is_intel_emmc, INTEL_EMMC_DEVICE_IDS};
pub use family::HardwareFamily;
pub use i2c_host::i2c_host;
#[cfg(any(
    feature = "nonos-capsule-driver-i2c-pci",
    feature = "nonos-capsule-driver-i2c-hid"
))]
pub use i2c_present::i2c_host_present;
pub use intel_ethernet_ids::{
    E1000E_DEVICE_IDS, E1000_DEVICE_IDS, IGB_DEVICE_IDS, IGC_DEVICE_IDS, IXGBE_DEVICE_IDS,
};
pub use missing::missing_path;
pub use present::present;
pub use realtek_ids::{
    RTL8139_DEVICE_ID, RTL8169_DEVICE_IDS, RTL8821CE_DEVICE_IDS, RTL8822CE_DEVICE_IDS,
};
pub use record::InventoryRecord;
pub use scan::scan;
pub use sd_host::{is_sd_host, INTEL_SDIO_DEVICE_IDS};
#[cfg(feature = "nonos-capsule-driver-spi")]
pub use spi_present::spi_host_present;
pub use state::SupportState;
pub use support::support_state;
pub use vmd::{is_intel_vmd, INTEL_VMD_DEVICE_IDS};
