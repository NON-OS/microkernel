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

use super::device::PciDevice;
use super::header::HeaderType;

impl PciDevice {
    pub fn supports_msi(&self) -> bool {
        self.msi.is_some()
    }

    pub fn supports_msix(&self) -> bool {
        self.msix.is_some()
    }

    pub fn is_pcie(&self) -> bool {
        self.pcie.is_some()
    }

    pub fn is_bridge(&self) -> bool {
        matches!(self.header_type, HeaderType::PciToPciBridge | HeaderType::CardBusBridge)
    }

    pub fn is_usb_controller(&self) -> bool {
        self.class_code.is_usb()
    }

    pub fn is_nvme_controller(&self) -> bool {
        self.class_code.is_nvme()
    }

    pub fn is_ahci_controller(&self) -> bool {
        self.class_code.is_ahci()
    }

    pub fn is_network_controller(&self) -> bool {
        self.class_code.is_network()
    }

    pub fn is_display_controller(&self) -> bool {
        self.class_code.is_display()
    }
}
