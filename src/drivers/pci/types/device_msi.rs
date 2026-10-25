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

// MSI and MSI-X delivery here is written in terms of the local APIC: the
// message address names an APIC ID and the message data carries an
// interrupt vector. An arm64 machine routes the same PCI messages through
// the GIC's ITS, addressed by device ID and translated through tables the
// interrupt controller owns, so none of this transfers. These four stay
// with the architecture whose message format they encode.
#[cfg(target_arch = "x86_64")]
impl super::device::PciDevice {
    pub fn configure_msix(
        &mut self,
        irq_vector: u8,
        dest_apic_id: u8,
    ) -> Result<(), crate::drivers::pci::error::PciError> {
        let msix =
            self.msix.as_ref().ok_or(crate::drivers::pci::error::PciError::MsixNotSupported)?;
        let config = crate::drivers::pci::config::ConfigSpace::new(self.address);
        crate::drivers::pci::msi::configure_msix(
            &config,
            msix,
            &self.bars,
            0,
            irq_vector,
            dest_apic_id,
        )?;
        crate::drivers::pci::msi::enable_msix(&config, msix)?;
        Ok(())
    }

    pub fn disable_msix(&mut self) -> Result<(), crate::drivers::pci::error::PciError> {
        let msix =
            self.msix.as_ref().ok_or(crate::drivers::pci::error::PciError::MsixNotSupported)?;
        let config = crate::drivers::pci::config::ConfigSpace::new(self.address);
        crate::drivers::pci::msi::disable_msix(&config, msix)
    }

    pub fn configure_msi(
        &mut self,
        irq_vector: u8,
        dest_apic_id: u8,
    ) -> Result<(), crate::drivers::pci::error::PciError> {
        let msi = self.msi.as_ref().ok_or(crate::drivers::pci::error::PciError::MsiNotSupported)?;
        let config = crate::drivers::pci::config::ConfigSpace::new(self.address);
        crate::drivers::pci::msi::configure_msi(&config, msi, irq_vector, dest_apic_id)
    }

    pub fn disable_msi(&mut self) -> Result<(), crate::drivers::pci::error::PciError> {
        let msi = self.msi.as_ref().ok_or(crate::drivers::pci::error::PciError::MsiNotSupported)?;
        let config = crate::drivers::pci::config::ConfigSpace::new(self.address);
        crate::drivers::pci::msi::disable_msi(&config, msi)
    }
}
