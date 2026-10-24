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

mod api;
pub mod bar;
mod bar_page;
pub mod capabilities;
// Config access is ECAM or the PC port pair, chosen at run time by what the
// platform published, so the module itself is not architecture specific.
pub mod config;
pub mod constants;
pub mod error;
pub mod manager;
// Capability and MSI-X table access is the same on every architecture; the
// message a function is given is not. The helpers that compose an x86 LAPIC
// message are re-exported below on x86_64 alone.
pub mod msi;
pub mod security;
pub mod stats;
pub mod types;
pub mod vmd;

pub use api::{
    pci_read_config32, pci_read_config32_safe, pci_write_config32, pci_write_config32_safe,
};
pub use bar::{decode_all_bars, decode_all_bars_unchecked, decode_bar, enumerate_bars, BarInfo};
pub use bar_page::with_bar_page;
pub use capabilities::{
    enumerate_capabilities, enumerate_pcie_capabilities, find_capability, get_msi_info,
    get_msix_info, get_pcie_info, get_power_management_info, has_capability, CapabilityWalker,
};
pub use config::{
    ecam, read32_unchecked, write32_unchecked, BridgeConfigSpace, ConfigSpace, EcamError,
    EcamWindow,
};
pub use error::{PciError, Result};
pub use manager::{
    count_devices, find_device_by_class, find_device_by_id, get_device_by_address,
    get_device_by_class, get_pci_manager, get_pci_stats, init_pci, is_initialized,
    scan_and_collect, scan_and_collect_safe, with_manager, PciManager,
};
#[cfg(target_arch = "x86_64")]
pub use msi::{
    disable_legacy_interrupt, disable_msi, disable_msix, enable_legacy_interrupt, enable_msix,
    is_msi_enabled, is_msix_enabled, mask_all_msix, mask_msi_vector, mask_msix_vector,
    unmask_all_msix, unmask_msi_vector, unmask_msix_vector,
};
pub use security::is_dma_capable;
pub use stats::PciStats;
pub use types::{
    BridgeInfo, ClassCode, DeviceId, HeaderType, MsiInfo, MsiMessage, MsixInfo, PciAddress, PciBar,
    PciCapability, PciDevice, PcieCapability, PcieDeviceType, PcieInfo, PowerManagementInfo,
};
