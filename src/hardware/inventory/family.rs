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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareFamily {
    StorageNvme,
    StorageAhci,
    StorageUsbMsc,
    StorageVirtioBlk,
    /// An Intel VMD: a PCI domain hiding NVMe and SATA controllers behind
    /// it (`vmd`). Listed, never spawned for.
    StorageVmd,
    /// An Intel eMMC host (`emmc`): the soldered disk of Atom, Celeron and
    /// Pentium Silver laptops. Served by the AHCI capsule for now.
    StorageEmmc,
    NetworkVirtio,
    NetworkE1000,
    NetworkRtl8139,
    NetworkRtl8169,
    NetworkRtl8821ce,
    NetworkIwlwifi,
    DisplayGopFramebuffer,
    DisplayVirtioGpu,
    DisplayBga,
    DisplayNativeIntel,
    DisplayNativeAmd,
    DisplayNativeNvidia,
    UsbXhci,
    UsbEhci,
    UsbOhci,
    UsbUhci,
    InputPs2,
    InputUsbHid,
    InputI2cHid,
    AudioHda,
    SerialI2c,
    SerialSpi,
    BridgePci,
    SystemPeripheral,
    Unknown,
}
