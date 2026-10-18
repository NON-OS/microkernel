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

use super::super::bar::decode_all_bars_unchecked;
use super::super::capabilities::{
    collect_all_capabilities, enumerate_pcie_capabilities, get_msi_info, get_msix_info,
    get_pcie_info, get_power_management_info,
};
use super::super::config::ConfigSpace;
use super::super::security::is_dma_capable;
use super::super::stats;
use super::super::types::{PciAddress, PciDevice};
use super::probe_header::read_header;

pub(super) use super::enumerate::enumerate_all_buses;

pub(crate) fn probe_device(bus: u8, device_num: u8, function: u8) -> Option<PciDevice> {
    let mut device = read_header(bus, device_num, function)?;
    let config = ConfigSpace::new(PciAddress::new(bus, device_num, function));

    let bars = decode_all_bars_unchecked(bus, device_num, function);

    let capabilities = collect_all_capabilities(&config).unwrap_or_default();
    let pcie_capabilities = enumerate_pcie_capabilities(bus, device_num, function);

    let msi = get_msi_info(&config).ok().flatten();
    let msix = get_msix_info(&config).ok().flatten();
    let power_management = get_power_management_info(&config).ok().flatten();
    let pcie = get_pcie_info(&config).ok().flatten();

    device.bars = bars;
    device.capabilities = capabilities;
    device.pcie_capabilities = pcie_capabilities;
    device.msi = msi;
    device.msix = msix;
    device.power_management = power_management;
    device.pcie = pcie;

    device.sync_compat_fields();

    stats::record_device_found(
        device.class(),
        device.vendor_id(),
        device.is_bridge(),
        device.is_pcie(),
        device.supports_msi(),
        device.supports_msix(),
        is_dma_capable(&device),
    );

    Some(device)
}
