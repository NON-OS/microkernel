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

extern crate alloc;

use alloc::vec::Vec;
use core::fmt;

use super::address::PciAddress;
use super::bar::PciBar;
use super::capability::{PciCapability, PcieCapability};
use super::class_code::ClassCode;
use super::device_id::DeviceId;
use super::header::HeaderType;
use super::msi::{MsiInfo, MsixInfo};
use super::pcie::PcieInfo;
use super::power::PowerManagementInfo;

#[derive(Clone, Debug)]
pub struct PciDevice {
    pub address: PciAddress,
    pub device_id_info: DeviceId,
    pub class_code: ClassCode,
    pub header_type: HeaderType,
    pub multifunction: bool,
    pub bars: [PciBar; 6],
    pub capabilities: Vec<PciCapability>,
    pub pcie_capabilities: Vec<PcieCapability>,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub msi: Option<MsiInfo>,
    pub msix: Option<MsixInfo>,
    pub power_management: Option<PowerManagementInfo>,
    pub pcie: Option<PcieInfo>,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub progif: u8,
}

impl fmt::Display for PciDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:04x}:{:04x} {} [{}]",
            self.address,
            self.device_id_info.vendor_id,
            self.device_id_info.device_id,
            self.class_code,
            self.class_code.name()
        )
    }
}
